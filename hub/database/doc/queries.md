# Live

## Electricity

### House Power Consumption

```sql
SELECT
  time,
  house_power as "Total",
  house_l1_power as "Power #1",
  house_l2_power as "Power #2",
  house_l3_power as "Power #3"
FROM electricity_consumption
WHERE
  $__timeFilter("time")
```

### Battery

```sql
SELECT
  time,
  ongoing_power as "Ongoing Power",
  voltage as "Voltage"
FROM electricity_storage
WHERE
  $__timeFilter("time")
```

### Production

```sql
SELECT
  time,
  power as "Total",
  l1_power as "Phase #1",
  l2_power as "Phase #2",
  l3_power as "Phase #3"
FROM electricity_production
WHERE
  $__timeFilter("time")
```

### State Of Charge

```sql
SELECT
  time,
  state_of_charge
FROM electricity_storage
WHERE
  $__timeFilter("time")
```

### Temperature

```sql
SELECT
  time,
  temperature
FROM electricity_storage
WHERE
  $__timeFilter("time")
```

## Ventilation

### Ventilation

```sql
SELECT
  time,
  supplied_temperature_after_ground_coupled_heat_exchanger as "Supplied after ground-coupled heat exchanger",
  supplied_temperature_after_heat_recovery_exchanger as "Supplied after heat recovery exchanger",
  extracted_temperature as "Extracted",
  discharged_temperature as "Discharged"
FROM air
WHERE
  $__timeFilter("time")
```

### Humidity

```sql
SELECT
  time,
  inside_humidity as "humidity"
FROM air
WHERE
  $__timeFilter("time")
```

## Domestic Hot Water

### Domestic Hot Water

```sql
SELECT
  time,
  top_of_the_tank_temperature as "Top of the tank",
  bottom_of_the_tank_temperature as "Bottom of the tank"
FROM domestic_hot_water
WHERE
  $__timeFilter("time")
```

# Daily View

## Temperatures

```sql
SELECT
  bucket_time as time,
  avg_extracted as "Average of extracted air",
  avg_supplied as "Average of supplied air"
FROM (
  SELECT
    time_bucket('1 day', time) as bucket_time,
    avg(extracted_temperature) as avg_extracted,
    avg(supplied_temperature_after_heat_recovery_exchanger) as avg_supplied
  FROM air
  WHERE
    $__timeFilter("time")
  GROUP BY bucket_time
  ORDER BY bucket_time DESC
) AS s
```

## Electricity consumption per day

```sql
SELECT
  time_bucket('1 day', s.t) as time,
  ROUND((sum(s.i * s.p) / 3600 / 1000)::numeric, 3) as "Consumption"
FROM (
  SELECT
    time as t,
    ROUND(EXTRACT(EPOCH FROM (time - lag(time) OVER ())::interval)) as i,
    (house_power + lag(house_power) OVER ()) / 2 as p
  FROM
    electricity_consumption
  WHERE
    $__timeFilter("time")
  ORDER BY time ASC
) AS s
GROUP BY time
ORDER BY time DESC
```

## Electricity production per day

```sql
SELECT
  time_bucket('1 day', s.t) as time,
  ROUND((sum(s.i * s.p) / 3600 / 1000)::numeric, 3) as "Production"
FROM (
  SELECT
    time as t,
    ROUND(EXTRACT(EPOCH FROM (time - lag(time) OVER ())::interval)) as i,
    (power + lag(power) OVER ()) / 2 as p
  FROM
    electricity_production
  WHERE
    $__timeFilter("time")
  ORDER BY time ASC
) AS s
GROUP BY time
ORDER BY time DESC
```
