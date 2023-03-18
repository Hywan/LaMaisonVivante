#!/bin/bash

ccsp_service_id="fdc85c00-0a2f-4c64-bcb4-2cfb1500730a"
ccsp_application_id="e7bcd186-a5fd-410d-92cb-6876a42288bd"

base_domain="prd.eu-ccapi.kia.com"
base_url="${base_domain}:8080"
spa_api_url="https://${base_url}/api/v1/spa"
user_api_url="https://${base_url}/api/v1/user"

stamp=$(curl -s "https://raw.githubusercontent.com/neoPix/bluelinky-stamps/master/kia-${ccsp_application_id}.v2.json" | jq '.stamps[42]')
echo -e "stamp: ${stamp}\n"

uuid=$(uuidgen)
echo -e "uuid: ${uuid}\n"

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
        --data "{\"pushRegId\": 1, \"pushType\": \"GCM\", \"uuid\": \"${uuid}\"}" \
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

login_form_url="https://eu-account.kia.com/auth/realms/eukiaidm/protocol/openid-connect/auth?client_id=f4d531c7-1043-444d-b09a-ad24bd913dd4&scope=openid%20profile%20email%20phone&response_type=code&hkid_session_reset=true&redirect_uri=${user_api_url}/integration/redirect/login&ui_locales=en&state=${service_id}:${user_id}"

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

redirect_url=$(
    curl \
        -s \
        --data "username=${KIA_USERNAME}" \
        --data "password=${KIA_PASSWORD}" \
        --data "credentialId=" \
        --data "rememberMe=on" \
        --cookie "${cookie_jar}" \
        --cookie-jar "${cookie_jar}" \
        --include \
        "${login_form_action_url}" | \
    rg --only-matching 'Location: ([^\r]+)' --replace '$1'
)

echo -e "redirect_url: ${redirect_url}\n"

login_form_action_url=$(
    curl \
        -s \
        --cookie "${cookie_jar}" \
        --cookie-jar "${cookie_jar}" \
        "${redirect_url}" | \
    rg --only-matching '<form.*action="([^"]+)"' --replace '$1' | \
    sed -e 's/&amp;/\&/g'
)

echo -e "login_form_action_url: ${login_form_action_url}\n"

redirect_url=$(
    curl \
        -s \
        --data "actionType=FIND" \
        --data "createToUVO=UVO" \
        --data "email=" \
        --cookie "${cookie_jar}" \
        --cookie-jar "${cookie_jar}" \
        --include \
        "${login_form_action_url}" | \
    rg --only-matching 'Location: ([^\r]+)' --replace '$1'
)

echo -e "redirect_url: ${redirect_url}\n"

user_id=$(
    curl \
        -s \
        --cookie "${cookie_jar}" \
        --cookie-jar "${cookie_jar}" \
        --include \
        "${redirect_url}" | \
    rg --only-matching 'Location: ([^\r]+)' --replace '$1' | \
    rg --only-matching '[&\?]int_user_id=([^&$]+)' --replace '$1'
)

echo -e "user_id: ${user_id}\n"

redirect_url=$(
    curl \
        -s \
        -H "ccsp-service-id: ${ccsp_service_id}" \
        -H 'Content-Type: application/json' \
        --data "{\"intUserId\": \"${user_id}\"}" \
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

access_token=$(echo ${token} | jq --raw-output '.access_token')
refresh_token=$(echo ${token} | jq --raw-output '.refresh_token')

echo "access_token: ${access_token}"
echo -e "refresh_token: ${refresh_token}\n"

token=$(
    curl \
        -s \
        -H "Authorization: Basic ZmRjODVjMDAtMGEyZi00YzY0LWJjYjQtMmNmYjE1MDA3MzBhOnNlY3JldA==" \
        -H "Stamp: ${stamp}" \
        --data 'grant_type=refresh_token' \
        --data 'redirect_uri=https://www.getpostman.com/oauth2/callback' \
        --data "refresh_token=${refresh_token}" \
        "${user_api_url}/oauth2/token"
)

refresh_token="$(echo ${token} | jq --raw-output '.token_type') $(echo ${token} | jq --raw-output '.access_token')"

echo -e "refresh_token: ${refresh_token}\n"
