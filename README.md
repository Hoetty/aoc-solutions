# My Advent of Code (AOC) Solutions
Check out advent of code at [https://adventofcode.com](https://adventofcode.com)

The solutions are written in Rust and cover mainly 2024, with all days available

## Running the program
My solutions can be run using ```cargo run```, altough using Release mode with ```cargo run -r``` is recommended.

Please place your inputs for the days you wish to run under ```./inputs/{year}/day{day}.txt```, for year 2024 day 3 this would be ```./inputs/2024/day3.txt```

The program accepts the following arguments:
```
Usage: aoc [OPTIONS]

Options:
  -y, --year <YEAR>  The years to run
  -d, --day <DAY>    The days to run
  -r, --redact       Redact solutions from output
  -o, --output       Output to ./output.txt in addition to the terminal
  -h, --help         Print help
  -V, --version      Print version
```

Please note that in order to select a day, a year is required

### Example
Run for year 2024 day 1, 2 and 3 

```sh
cargo run -r -- -y 2024 -d 1 -d 2 -d 3 -r
```

### Output
```
======= ==== =========== === ========= =========
              Year 2024                         
======= ==== =========== === ========= =========
 Day 1    I                   0.073ms    12.78% 
         #1    ######     ✔   0.027ms     4.73% 
         #2    ######     ✔   0.019ms     3.33% 
                          ✔   0.119ms    20.84% 
                                                
 Day 2    I                   0.131ms    22.94% 
         #1    ######     ✔   0.029ms     5.08% 
         #2    ######     ✔   0.172ms    30.12% 
                          ✔   0.332ms    58.14% 
                                                
 Day 3    I                   0.015ms     2.63% 
         #1    ######     ✔   0.049ms     8.58% 
         #2    ######     ✔   0.056ms     9.81% 
                          ✔   0.120ms    21.02% 
                                                
======= ==== =========== === ========= =========
 Total                    ✔   0.571ms   100.00% 
======= ==== =========== === ========= =========
```

The output shows a breakdown of all solutions (here redacted), the time taken for the input, part 1, part 2 and all combined as well as the percantage of the total time.
Solutions are also checked against expected solutions.

### Testing
My solutions also allow for providing the correct solution and testing against it, for example if you tinker with the algorithm and want to see if changes to it broke the logic

To use testing, you can put your two expected results seperated by a new line into ```./expect/{year}/day{day}.txt```
For example, you could write 
```
153
5353553535353
```
into ```./expect/2024/day1.txt```

### Performance
My solutions are optimized for both performance and code readability. The code is not parallelized, it just runs on one core.

For my setup, the entire year 2024 completes in around 40ms (if you want to see really fast solutions, then check out [this cool repo](https://github.com/indiv0/aoc-fastest))
```
======== ==== =========== === ========== =========
               Year 2024                          
======== ==== =========== === ========== =========
  Day 1    I                    0.059ms     0.15% 
          #1    ######     ✔    0.027ms     0.07% 
          #2    ######     ✔    0.018ms     0.05% 
                           ✔    0.104ms     0.26% 
                                                  
  Day 2    I                    0.174ms     0.44% 
          #1    ######     ✔    0.037ms     0.09% 
          #2    ######     ✔    0.195ms     0.50% 
                           ✔    0.406ms     1.03% 
                                                  
  Day 3    I                    0.019ms     0.05% 
          #1    ######     ✔    0.055ms     0.14% 
          #2    ######     ✔    0.057ms     0.15% 
                           ✔    0.131ms     0.33% 
                                                  
  Day 4    I                    0.033ms     0.08% 
          #1    ######     ✔    0.175ms     0.45% 
          #2    ######     ✔    0.091ms     0.23% 
                           ✔    0.299ms     0.76% 
                                                  
  Day 5    I                    0.098ms     0.25% 
          #1    ######     ✔    0.016ms     0.04% 
          #2    ######     ✔    0.047ms     0.12% 
                           ✔    0.161ms     0.41% 
                                                  
  Day 6    I                    0.244ms     0.62% 
          #1    ######     ✔    0.000ms     0.00% 
          #2    ######     ✔    0.574ms     1.46% 
                           ✔    0.818ms     2.08% 
                                                  
  Day 7    I                    0.198ms     0.50% 
          #1    ######     ✔    0.072ms     0.18% 
          #2    ######     ✔    0.155ms     0.39% 
                           ✔    0.425ms     1.08% 
                                                  
  Day 8    I                    0.029ms     0.07% 
          #1    ######     ✔    0.009ms     0.02% 
          #2    ######     ✔    0.030ms     0.08% 
                           ✔    0.068ms     0.17% 
                                                  
  Day 9    I                    0.010ms     0.03% 
          #1    ######     ✔    0.487ms     1.24% 
          #2    ######     ✔    0.283ms     0.72% 
                           ✔    0.780ms     1.99% 
                                                  
 Day 10    I                    0.056ms     0.14% 
          #1    ######     ✔    0.009ms     0.02% 
          #2    ######     ✔    0.001ms     0.00% 
                           ✔    0.066ms     0.17% 
                                                  
 Day 11    I                    0.076ms     0.19% 
          #1    ######     ✔    0.001ms     0.00% 
          #2    ######     ✔    1.833ms     4.67% 
                           ✔    1.910ms     4.86% 
                                                  
 Day 12    I                    0.082ms     0.21% 
          #1    ######     ✔    0.294ms     0.75% 
          #2    ######     ✔    0.319ms     0.81% 
                           ✔    0.695ms     1.77% 
                                                  
 Day 13    I                    0.069ms     0.18% 
          #1    ######     ✔    0.007ms     0.02% 
          #2    ######     ✔    0.008ms     0.02% 
                           ✔    0.084ms     0.21% 
                                                  
 Day 14    I                    0.057ms     0.15% 
          #1    ######     ✔    0.009ms     0.02% 
          #2    ######     ✔    0.138ms     0.35% 
                           ✔    0.204ms     0.52% 
                                                  
 Day 15    I                    0.137ms     0.35% 
          #1    ######     ✔    0.077ms     0.20% 
          #2    ######     ✔    0.333ms     0.85% 
                           ✔    0.547ms     1.39% 
                                                  
 Day 16    I                    0.144ms     0.37% 
          #1    ######     ✔    0.001ms     0.00% 
          #2    ######     ✔    0.009ms     0.02% 
                           ✔    0.154ms     0.39% 
                                                  
 Day 17    I                    0.009ms     0.02% 
          #1    ######     ✔    0.001ms     0.00% 
          #2    ######     ✔    0.181ms     0.46% 
                           ✔    0.191ms     0.49% 
                                                  
 Day 18    I                    0.115ms     0.29% 
          #1    ######     ✔    0.030ms     0.08% 
          #2    ######     ✔    0.097ms     0.25% 
                           ✔    0.242ms     0.62% 
                                                  
 Day 19    I                    0.786ms     2.00% 
          #1    ######     ✔    0.000ms     0.00% 
          #2    ######     ✔    4.255ms    10.84% 
                           ✔    5.041ms    12.84% 
                                                  
 Day 20    I                    0.041ms     0.10% 
          #1    ######     ✔    0.043ms     0.11% 
          #2    ######     ✔    5.684ms    14.48% 
                           ✔    5.768ms    14.69% 
                                                  
 Day 21    I                    0.011ms     0.03% 
          #1    ######     ✔    0.008ms     0.02% 
          #2    ######     ✔    0.047ms     0.12% 
                           ✔    0.066ms     0.17% 
                                                  
 Day 22    I                   15.905ms    40.51% 
          #1    ######     ✔    0.000ms     0.00% 
          #2    ######     ✔    0.000ms     0.00% 
                           ✔   15.905ms    40.51% 
                                                  
 Day 23    I                    0.132ms     0.34% 
          #1    ######     ✔    0.387ms     0.99% 
          #2    ######     ✔    4.323ms    11.01% 
                           ✔    4.842ms    12.33% 
                                                  
 Day 24    I                    0.042ms     0.11% 
          #1    ######     ✔    0.007ms     0.02% 
          #2    ######     ✔    0.027ms     0.07% 
                           ✔    0.076ms     0.19% 
                                                  
 Day 25    I                    0.079ms     0.20% 
          #1    ######     ✔    0.201ms     0.51% 
          #2    ######     ✔    0.001ms     0.00% 
                           ✔    0.281ms     0.72% 
                                                  
======== ==== =========== === ========== =========
  Total                    ✔   39.264ms   100.00% 
======== ==== =========== === ========== =========
```

Output created with the following command: 
```sh
cargo run -r -- -y 2024 -r -o
```

### Visualisations
There is currently a little terminal visualization for the concept behind year 2024 day 15 under ./visualizations. More might be added in the future.

### Disclaimer
All solutions work on my input. I do not guarantee, that they will work for your input. If i missed an edge case, that is present in your input it may not give the correct result. You are free to open an issue, if you find a bug.
