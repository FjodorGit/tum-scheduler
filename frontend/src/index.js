import "./styles/root.css";
import "flowbite";

const configureBtn = document.querySelector("#configure-courses-btn");
const configureWindow = document.querySelector("#configure-window");
const departmentTemplate = document.querySelector("#department-template");
const departmentList = document.querySelector("#department-list");

function add_department(department_name) {
  let departmentClone = departmentTemplate.content.cloneNode(true);
  let departmentLabel = departmentClone.querySelector("label");
  departmentLabel.textContent = department_name;
  departmentList.appendChild(departmentClone);
}

async function fetch_departments() {
  const response = await fetch("/api/departments");
  const departments = await response.json();
  departments.forEach((department) => {
    let department_name = department.split(" ");
    department_name.shift();
    department_name.shift();
    add_department(department_name.join(" "));
  });
  console.log(departments);
}

async function fetch_courses(configuration) {}

addEventListener("DOMContentLoaded", (_) => {
  const weeksidebar = document.querySelector("#weekday-sidebar");

  // Calculate number of lines
  const numberOfLines = weeksidebar.clientHeight / 50;
  const sidebarNumberTemplate = document.querySelector("#sidebar-number");
  // Loop to create and append numbers
  for (let i = 5; i < 4 + numberOfLines; i++) {
    const sidebarNumber =
      sidebarNumberTemplate.content.cloneNode(true).children[0];
    if (i < 13) {
      sidebarNumber.textContent = i + " AM";
      if (i === 5) {
        sidebarNumber.style.marginTop = "6px";
      }
    } else {
      sidebarNumber.textContent = (i % 12) + " PM";
    }
    weeksidebar.appendChild(document.importNode(sidebarNumber, true));
  }
});

configureBtn.addEventListener("mousedown", async (_) => {
  console.log("Open configuration");
  await fetch_departments();
  if (configureWindow.classList.contains("-translate-y-full")) {
    configureWindow.classList.remove("-translate-y-full");
  } else {
    configureWindow.classList.add("-translate-y-full");
  }
});
