import "./styles/root.css";
import "flowbite";
import { setupCourseConfiguration } from "./course_configuration";

const configureBtn = document.querySelector("#configure-courses-btn");
const configureWindow = document.querySelector("#configure-window");
const departmentTemplate = document.querySelector("#department-template");
const departmentList = document.querySelector("#department-list");

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
  setupCourseConfiguration();
});

configureBtn.addEventListener("mousedown", async (_) => {
  console.log("Open configuration");
  if (configureWindow.classList.contains("-translate-y-full")) {
    configureWindow.classList.remove("-translate-y-full");
  } else {
    configureWindow.classList.add("-translate-y-full");
  }
});
