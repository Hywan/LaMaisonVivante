#!/bin/bash

ccsp_service_id="fdc85c00-0a2f-4c64-bcb4-2cfb1500730a"
ccsp_application_id="a2b8469b-30a3-4361-8e13-6fceea8fbe74"

base_domain="prd.eu-ccapi.kia.com"
base_url="${base_domain}:8080"
spa_api_url="https://${base_url}/api/v1/spa"
user_api_url="https://${base_url}/api/v1/user"

# generate stamp with:

# use base64::prelude::*;
# use std::time::{SystemTime, UNIX_EPOCH};

# const APP_ID: &str = "a2b8469b-30a3-4361-8e13-6fceea8fbe74";
# const CFB: &str = "wLTVxwidmH8CfJYBWSnHD6E0huk0ozdiuygB4hLkM5XCgzAL1Dk5sE36d/bx5PFMbZs=";

# fn main() {
#     println!("Hello, world!");
#     let now = SystemTime::now()
#         .duration_since(UNIX_EPOCH)
#         .unwrap()
#         .as_secs();

#     let raw_data = format!("{APP_ID}:{now}");
#     let cfb = BASE64_STANDARD.decode(CFB).unwrap();

#     let bytes = cfb
#         .into_iter()
#         .zip(raw_data.into_bytes().into_iter())
#         .map(|(b1, b2)| b1 ^ b2)
#         .collect::<Vec<u8>>();

#     let result = BASE64_STANDARD.encode(bytes);

#     dbg!(&result);
# }

# stamp=$(curl -s "https://raw.githubusercontent.com/neoPix/bluelinky-stamps/master/kia-${ccsp_application_id}.v2.json" | jq '.stamps[42]')
stamp='oYa3/zyroR0vT6ZgagTzPJcFq9FRkgRPjU5ih3eFC/Og5gc/7ggOgHTPQsXF08E='
echo -e "stamp: ${stamp}\n"

uuid=$(uuidgen | tr '[:upper:]' '[:lower:]')
echo -e "uuid: ${uuid}\n"

push_reg_id=$(LC_ALL='C' tr -dc A-Za-z0-9 < /dev/urandom | head -c 64)
echo -e "push_reg_id: ${push_reg_id}"

user_agent_ok="okhttp/3.12.0"
user_agent_mozilla="Mozilla/5.0 (Linux; Android 4.1.1; Galaxy Nexus Build/JRO03C) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Mobile Safari/535.19"

device_id=$(
    curl \
        -s \
        -H "ccsp-service-id: ${ccsp_service_id}" \
        -H "ccsp-application-id: ${ccsp_application_id}" \
        -H "Stamp: ${stamp}" \
        -H "Content-Type: application/json" \
        -H "Host: ${base_url}" \
        -H "User-Agent: ${user_agent_ok}" \
        --data "{\"pushRegId\": \"${push_reg_id}\", \"pushType\": \"APNS\", \"uuid\": \"${uuid}\"}" \
        "${spa_api_url}/notifications/register" | \
    jq --raw-output '.resMsg.deviceId'
)

echo "device_id: ${device_id}"

cookie_jar="cookies.txt"

curl \
    -s \
    --cookie-jar "${cookie_jar}" \
    "${user_api_url}/oauth2/authorize?response_type=code&state=test&client_id=${ccsp_service_id}&redirect_uri=${user_api_url}/oauth2/redirect&lang=en" > /dev/null

cat "${cookie_jar}"

curl \
    -s \
    -H 'Content-Type: application/json' \
    --data '{"lang": "en"}' \
    --cookie "${cookie_jar}" \
    "${user_api_url}/language" > /dev/null

echo -e "set language exit code: ${?}\n"

integration_info=$(
    curl \
        -s \
        --cookie "${cookie_jar}" \
        --cookie-jar "${cookie_jar}" \
        "${user_api_url}/integrationinfo"
)

user_id=$(echo $integration_info | jq --raw-output '.userId')
service_id=$(echo $integration_info | jq --raw-output '.serviceId')

echo "user_id: ${user_id}"
echo -e "service_id: ${service_id}\n"

auth_client="572e0304-5f8d-4b4c-9dd5-41aa84eed160"
login_form_url="https://eu-account.kia.com/auth/realms/eukiaidm/protocol/openid-connect/auth?client_id=${auth_client}&scope=openid%20profile%20email%20phone&response_type=code&hkid_session_reset=true&redirect_uri=${user_api_url}/integration/redirect/login&ui_locales=en&state=${service_id}:${user_id}"

echo -e "login_form_url: ${login_form_url}\n"

login_form_action_url=$(
    curl \
        -s \
        --cookie "${cookie_jar}" \
        --cookie-jar "${cookie_jar}" \
        "${login_form_url}" | \
    rg --only-matching '<form.*action="([^"]+)"' --replace '$1' | \
    sed -e 's/&amp;/\&/g'
)

echo -e "login_form_action_url: ${login_form_action_url}\n"

echo -e "KIA_USERNAME: ${KIA_USERNAME}\n"
echo -e "KIA_PASSWORD: ${KIA_PASSWORD}\n"

# Use `--location` to automatically follow redirections (code 302).
curl \
    -s \
    --data "username=${KIA_USERNAME}" \
    --data "password=${KIA_PASSWORD}" \
    --data "credentialId=" \
    --data "rememberMe=on" \
    --cookie "${cookie_jar}" \
    --cookie-jar "${cookie_jar}" \
    --include \
    --location "${login_form_action_url}"

redirect_url=$(
    curl \
        -s \
        -H "ccsp-service-id: ${ccsp_service_id}" \
        -H 'Content-Type: application/json' \
        --data "{\"intUserId\": \"0\"}" \
        --cookie "${cookie_jar}" \
        --cookie-jar "${cookie_jar}" \
        "${user_api_url}/silentsignin" | \
    jq --raw-output '.redirectUrl'
)

echo -e "redirect_url: ${redirect_url}\n"


authorization_code=$(
    echo ${redirect_url} | \
    rg --only-matching '[&\?]code=([^&$]+)' --replace '$1'
)

echo -e "authorization_code: ${authorization_code}\n"

token=$(
    curl \
        -s \
        -H "Authorization: Basic ZmRjODVjMDAtMGEyZi00YzY0LWJjYjQtMmNmYjE1MDA3MzBhOnNlY3JldA==" \
        --data 'grant_type=authorization_code' \
        --data "redirect_uri=${user_api_url}/oauth2/redirect" \
        --data "code=${authorization_code}" \
        "${user_api_url}/oauth2/token"
)

echo $token

token_type=$(echo ${token} | jq --raw-output '.token_type')
access_token="${token_type} $(echo ${token} | jq --raw-output '.access_token')"
authorization_code=$(echo ${token} | jq --raw-output '.refresh_token')

echo "access_token: ${access_token}"
echo -e "authorization_code: ${authorization_token}\n"

token=$(
    curl \
        -s \
        -H "Authorization: Basic ZmRjODVjMDAtMGEyZi00YzY0LWJjYjQtMmNmYjE1MDA3MzBhOnNlY3JldA==" \
        -H "Stamp: ${stamp}" \
        --data 'grant_type=refresh_token' \
        --data 'redirect_uri=https://www.getpostman.com/oauth2/callback' \
        --data "refresh_token=${authorization_code}" \
        "${user_api_url}/oauth2/token"
)

refresh_token="$(echo ${token} | jq --raw-output '.token_type') $(echo ${token} | jq --raw-output '.access_token')"

echo -e "refresh_token: ${refresh_token}\n"

curl \
    --verbose \
    -H "Authorization: ${access_token}" \
    -H "ccsp-service-id: ${ccsp_service_id}" \
    -H "ccsp-application-id: ${ccsp_application_id}" \
    -H "ccsp-device-id: ${device_id}" \
    -H "Connection: Keep-Alive" \
    -H "Accept-Encoding: gzip" \
    -H "Stamp: ${stamp}" \
    -H "Host: ${base_url}" \
    -H "User-Agent: ${user_agent_ok}" \
    "${spa_api_url}/vehicles"
