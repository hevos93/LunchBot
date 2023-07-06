## LunchBot
### Description
The LunchBot is an API that will send information regarding the most important lunch task of the day, specifically lunch.

### Environmental Variables
These variables can be changed in a docker-compose file to change how the API acts.

| Variable | Default Value | Description | Alternatives | 
| --- | --- | --- | --- |
| LUNCHBOT_PORT | 4000 | This is the port where the API can receive requests. | Any port over 1024 |
| RUST_LOG | info | The level of logging | trace, debug, info, info, warn, error | 