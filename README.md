# tum-scheduler
**Optimize your next Semester at TUM.**

# Problems
Choosing courses at TUM for a semester can be overwhelming, especially in the master's program where there are no mandatory courses. 
Most courses have lecture and exercise classes, and it's common to attend all lectures but only one exercise session per course. 
However, on the TUM online platform, each part is treated as a separate course. On top of that, students have other weekly commitments like work or sports which should be factored in when designing a schedule. 
Designing an optimal schedule becomes even more challenging when other appointments are variable, such as when working students must work two days per week but can choose which days.
So, it's tough to create an optimal schedule that avoids clashes between lectures, exercises, and personal commitments.
Moreover, the TUM web platform for selecting courses presents minor inconveniences, such as a cumbersome search functionality or having to consult an additional website for missing course descriptions.

# Solution
Introducing an innovative application designed to streamline the process of creating a weekly schedule for an entire semester while seamlessly integrating personal commitments to avoid any overlaps. This application not only simplifies course selection but also enhances the browsing experience on the TUM platform.
Key features include: 

- Customizable optimization options: Tailor your schedule according to specific preferences such as credit requirements or the number of courses, while accommodating constraints like minimum weekly workdays. For instance, if you need to work at least two days a week and aim to achieve a minimum of 30 credits in a semester, the application can optimize your schedule accordingly.

- Automated selection of course components: Simplify course selection by automatically including all related lecture and exercise classes when choosing a course.

- Comprehensive course information: Addressing the common issue of missing course descriptions, the application provides access to detailed information directly within the platform, eliminating the need to consult external sources like the module catalogue.

- Enhanced search functionality: Enjoy an improved browsing experience with enhanced search capabilities, making it easier to find and select courses based on your preferences and requirements.

# Installation and Usage
The web frontend is still in progress, so the way to curretly use the tum-scheduler api is through docker.
Since the container contains a Gurobi solver a [Gurobi WLS-License](https://www.gurobi.com/features/web-license-service/) is required to run the docker image.
Clone this repo
```
git clone git@github.com:FjodorGit/tum-scheduler.git && cd tum-scheduler
```
and pull the web server image from the regestry
```
sudo docker image pull fkholodkov/tum-scheduler:latest
```
Now create the ```$GUROBI_LIC``` environment variable and point it to the path of your [Gurobi WLS-License](https://www.gurobi.com/features/web-license-service/)
```
export GUROBI_LIC=/path/to/gurobi.lic
```
and run the application with ```docker compose```
```
sudo -E docker compose up
```
The  ```-E``` flag required for the docker-compose.yaml file to read the exported environment variable.
This will spin up two containers. One for the server itself and one for the database. This command also automatically populates the database with course data from the current semester.

Now all is left to do is send requests to the api throgh ones favorite tool e.g  cURL or Postman.
Here is an example request through cURL.:
```
curl -H "Content-Type: application/json" --data-binary @resources/example.json 'http://172.17.0.1:8080/api/optimize' | json_pp
```


# Implementation and Tech Stack
The applications backend is written in Rust and comprises a scraper, a PostgreSQL database, and an Actix-web server
![alt text](https://github.com/FjodorGit/tum-scheduler/blob/main/resources/tum-scheduler-arch.png "Rough outline of the applications architecture")

### Scraper
By reverse engineering the TUM web API, it's possible to retrieve all available courses per semester. 
Each course has specific endpoints that need to be called to fetch particular information such as timing or descriptions. 
The Rust ORM Diesel is used to interact with a PostgreSQL database.

### Scheduler
Optimizing schedules is achieved by modeling the problem using (binary) integer programming. 
Gurobi is employed as the solver for the problem, with the rust_grb crate facilitating communication with the Gurobi API.

### Web Server
A simple Actix-web server serves as a thin wrapping layer to communicate with the scheduler in the backend.

