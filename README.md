# Test task for UnitedTraders company
## Actix-Web backend
### Written in a simplified gitflow
This is a REST API Webserver for organizing logging.

The entire application works in a Docker container. To build it:

    sudo docker-compose up -d

It is connected to nginx with open port 80 for http connections to the localhost, postgresql version 12.6, and the backend itself.

The frontend will be ready soon, but this the project actualy shows backend&DevOps skills.
HTML files are available on localhost/.
The sources are in /static.
If file is not found - returns /index.html.

### Checking if the server is alive:
    curl -X GET http://localhost/api/health

### Checking backend version:
    curl -X GET http://localhost/api/version

### Adding new log:
    curl -X POST \
    -H "Content-type: application/json" \
    -d '{"message": "Something happened", "level": "Warning"}' \
    http://localhost/api/logs

### Getting all logs:
    curl -X GET http://localhost/api/logs

### Getting logs in some range with specific logging level:
    curl -X GET 'http://localhost/api/logs/range?from_time=2015-10-15T18:27:01.002345&to_time=2021-05-21T21:08:09.345678&logging_level=Warning'

Params and their format:
- message   -> String
- level     -> only 'Warning', 'Error', 'Debug' or 'Info' 
- from_time -> yyyy-mm-ddThh:mm:ss.micsec
- to_time   -> similar to from_time
- logging_level -> similar to level
