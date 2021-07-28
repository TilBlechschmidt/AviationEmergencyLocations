# Turn radius
To calculate radius of turns, use this formula
```velocity^2 / (9.81 * tan(bankAngle))```
where velocity is in m/s and bankAngle in whatever the tan function takes

# Altitude loss during a turn while gliding
Percentage of a full turn when doing an "impossible turn" (180º followed by two 30º turns):
```(180+30+30)/360 = 66,66666%```
Radius of turn at 45º and 76kt:
-> To make a safe turn at 45º the airspeed has to be increased to at least 1.44\*Vs (1.5\*Vs is recommended)
```(velocity*1.44)^2 / (9.81 * tan(bankAngle)) = 323m = 1060ft```
Circumference of circle:
```2 * π * 323m = 2030m```
Distance covered during circle (2780m is the glide distance per 1000ft height loss for the C172):
```2030m * 67% = 1360.1m (or 49% of 2780m)```
Height loss to glide distance (convert to actual feet height loss):
```1000ft * 0.49 = 490ft```

Comparing against numbers from a [brilliant AOPA article](https://www.aopa.org/news-and-media/all-news/2002/july/pilot/engine-out) yields an error of about 8% (450ft vs. 490ft). While off by a bit, it errors on the side of caution and provides a reasonable safety margin.

Using `1.5\*Vs` yields a height loss of 530ft which is a whopping ~8% increase from `1.44\*Vs`!

# Calculating climb factor
Given climb rate and airspeed:
```yaml
climb:
    rate: 3.09
    speed: 79
```
The speed in KIAS (ignorantly skipping KCAS) is equal to KTAS in a standard atmosphere at MSL.
Speed converted to m/s:
```40,67m/s```
Duration to climb to 1000m:
```1000m / 3.09m/s = 323,62s```
Distance travelled diagonally:
```40,67m/s * 323,62s = 13161.6254m```
Calculating the ground track via pythagoras:
```sqrt(1000m^2 + 13161.6254m^2) ~= 13200m```
Converting to a slope:
```13200m / 1000m = 13.2```
For each X height gained, slope * X ground distance is covered.

Example: Climbing 2.000ft -> 2000ft * ~13.2 = 26.400ft = 4.344879nm
POH: 4nm

More comparisons:
| Height | Calculation | POH |
| ------ | ----------- | --- |
| 1000   | 2.17244     | 2   |
| 2000   | 4.344879    | 4   |
| 3000   | 6.517319    | 6   |
| 4000   | 8.689758    | 8   |

=> It appears that the POH is rounding and that past 3000ft the reduced climb-rate is affecting accuracy
    (albeit into the direction of more tolerance in terms of having an engine failure)

# Calculating glide factor
Calculate ratio between 1000ft (304.8m) and distance (example C172):
```2800m / 304.8m = 9.18```

# TODOs
- Write a tool to generate "derived" values
    - Climp factor (e.g. 13.2 for C172)
    - Glide factor (e.g. 9.18 for C172)
