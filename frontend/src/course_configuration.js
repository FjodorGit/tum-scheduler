import { MeiliSearch } from "meilisearch";

const departmentTemplate = document.querySelector("#department-template");
const departmentList = document.querySelector("#department-list");
const semesterRadios = document.querySelectorAll(".semesterRadio");

const courseSearch = document.querySelector("#course-search");
courseSearch.addEventListener("input", (event) => course_search(event));

const courseResults = document.querySelector("#course-results");
const courseResultTemplate = document.querySelector("#course-result-template");

class CourseRequest {
  departments = [];
  semester = "24S";

  add_department(department) {
    this.departments.push(department);
  }
}
const courseRequestConfiguration = new CourseRequest();

const mlsClient = new MeiliSearch({
  host: "http://localhost:7700",
  apiKey: "masterKey",
});

async function fetch_courses() {
  let request = JSON.stringify(courseRequestConfiguration);
  console.log(request);
}

async function set_semester(pointer_event) {
  let selectedSemester = pointer_event.target.value;
  if (courseRequestConfiguration.semester !== selectedSemester) {
    courseRequestConfiguration.semester = selectedSemester;
    await fetch_courses();
  } else {
    courseRequestConfiguration.semester = selectedSemester;
  }
}

async function set_department(pointer_event) {
  let selectedDepartment = pointer_event.target.value;
  let departmentIndex =
    courseRequestConfiguration.departments.indexOf(selectedDepartment);
  if (departmentIndex === -1) {
    courseRequestConfiguration.departments.push(selectedDepartment);
    await fetch_courses();
  } else {
    courseRequestConfiguration.departments.splice(departmentIndex, 1);
    await fetch_courses();
  }
}

async function fetch_departments() {
  const response = await fetch("/api/departments");
  const departments = await response.json();
  departments.forEach((department) => {
    let departmentClone = departmentTemplate.content.cloneNode(true);
    let departmentCheckbox = departmentClone.querySelector("input");
    departmentCheckbox.onclick = set_department;
    departmentCheckbox.value = department;
    let departmentName = department.split(" ");
    departmentName.shift();
    departmentName.shift();
    let departmentLabel = departmentClone.querySelector("label");
    departmentCheckbox.id = departmentName.join("-");
    departmentLabel.setAttribute("for", departmentName.join("-"));
    departmentLabel.textContent = departmentName.join(" ");
    departmentList.appendChild(departmentClone);
  });
}

function render_searched_courses(searchResults) {
  searchResults.forEach((res) => {
    let courseResultClone = courseResultTemplate.content.cloneNode(true);
    courseResultClone.querySelector("#result-subject").textContent =
      res.subject;
    courseResultClone.querySelector("#result-name").textContent = res.name_en;
    courseResults.appendChild(courseResultClone);
  });
}

function course_search(event) {
  courseResults.innerHTML = "";
  mlsClient
    .index("lectures")
    .search(courseSearch.value)
    .then((queryResponse) => render_searched_courses(queryResponse.hits));
}

export async function setupCourseConfiguration() {
  await fetch_departments();

  semesterRadios.forEach((elem) =>
    elem.addEventListener("click", set_semester),
  );
}
