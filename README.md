# tum-scheduler
**Optimize your next Semester at TUM.**

# Problems
Choosing courses at TUM for a semester can be overwhelming, especially in the master's program where there are no mandatory courses. 
Most courses have lecture and exercise classes, and it's common to attend all lectures but only one exercise session per course. 
However, on the TUM online platform, each part is treated as a separate course. On top of that, students have other weekly commitments like work or sports which should be factored in when designing a schedule. 
Designing an optimal schedule becomes even more challenging when other appointments are variable, such as when working students must work two days per week but can choose which days.
So, it's tough to create an optimal schedule that avoids clashes between lectures, exercises, and personal commitments.

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
![alt text](https://github.com/FjodorGit/tum-scheduler/blob/main/resources/tum-scheduler-arch.png "Rough outline of the applications architecture")
## Scraper
## Schedular



