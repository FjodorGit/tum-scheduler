# tum-scheduler
Optimize your schedule for a Semester at TUM.

# Problems
There are a lot of courses to choose from in a semester at TUM. Especially in the masters degree where you have no courses you are obligated to take.
Most of the courses consist of lecture and exercise classes. Typically you would want to visit each lecture but only one exercise class. 
On the TUM online platform these are considered two different courses.
Addtionally one would have other weekly appointments to fit into the weekly schedule (like work or sport training).
For working students it is most of the time possible to choose the days or hours the work at.
So it is challanging to choose a selection of courses that so that neither of the lecture or exercise classes or your personal schedule overlap.

# Solution
An application that can automatically create a weekly schedule for a semester considering ones personal schedule without overlap.
Additionally giving an improved platform for browsing and choosing courses that you consider taking during the semester.
Specifically the improvements are:
- option to optimize the schedule with respect to some specific metric like credits or amount of courses while accounting for bounding conditions like minimum number of days. Example: For you job you have to work a minimum of two days per week, but you would also like gain a minimum of 30 credits that semester.
- when selecting a course automatically select all lecture and exercise classes
- the course description is often missing for the course. So most of the time you have to consult the module catalogue to understand what the course is about
- better search through courses

# Usage
Docker
Frontend in progress

# Implementation
![alt text](https://github.com/FjodorGit/tum-scheduler/resources/tum-scheduler-arch.png "Rough outline of the applications architecture")
## Scraper
## Schedular



