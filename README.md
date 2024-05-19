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
The web frontend is currently under development, so to utilize the tum-scheduler API, Docker is the recommended method. 
Please note that since the container includes a Gurobi solver, a [Gurobi WLS-License](https://www.gurobi.com/features/web-license-service/)  is required to run the Docker image.
First, clone this repository:
```
git clone git@github.com:FjodorGit/tum-scheduler.git && cd tum-scheduler
```
Next, pull the web server image from the registry:
```
sudo docker image pull fkholodkov/tum-scheduler:latest
```
Now, create the ```$GUROBI_LIC``` environment variable and set it to the path of your [Gurobi WLS-License](https://www.gurobi.com/features/web-license-service/):
```
export GUROBI_LIC=/path/to/gurobi.lic
```
Run the application using Docker Compose:
```
sudo -E docker compose up
```
The ```-E``` flag is necessary for the docker-compose.yaml file to read the exported environment variable. This command will start two containers, one for the server and one for the database. Additionally, it will automatically populate the database with course data for the current semester.

To interact with the API, you can use your preferred tool such as cURL or Postman to send a request to the container. Here's an example cURL request:
```
curl -H "Content-Type: application/json" --data-binary @resources/maxects_example.json 'http://172.17.0.1:8080/api/optimize' | json_pp
```
This example requests an optimization for the curriculum of a Mathematics Master student and tries to find a schedule out of all Mathematics master courses available at TUM. 
It does so while maximizing the number of ECTS credits of the schedule and ensuring that courses have to be taken only two days per week.
Another example ([_mincourses_example_](https://github.com/FjodorGit/tum-scheduler/blob/main/resources/mincourses_example.json)) would be to minimize the number of courses a student has to take while having at least a required number of ECTS that semester. Refer [here](https://github.com/FjodorGit/tum-scheduler/blob/main/resources/api_docu.yaml) for the full endpoint documentation.

# Implementation and Tech Stack
The applications backend is written in Rust and comprises a scraper, a PostgreSQL database, and an [actix-web](https://actix.rs/) server

<p align="center">
  <img src="https://github.com/FjodorGit/tum-scheduler/blob/main/resources/tum-scheduler-arch.png">
</p>

### Scraper
By reverse engineering the TUM web API, it's possible to retrieve all available courses per semester. 
Each course has specific endpoints that need to be called to fetch particular information such as timing or descriptions. 
The Rust ORM [Diesel](https://diesel.rs/) is used to interact with a PostgreSQL database.

### Scheduler
Optimizing schedules is achieved by modeling the problem using (binary) integer programming.
[Gurobi](https://www.gurobi.com/) is employed as the solver for the problem, with the [rust_grb crate](https://crates.io/crates/grb/2.0.0) facilitating communication with the Gurobi API.

### Web Server
A simple [actix-web](https://actix.rs/) server serves as a thin wrapping layer to communicate with the scheduler in the backend.

