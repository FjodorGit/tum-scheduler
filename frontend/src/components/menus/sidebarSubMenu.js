import { createPrefixTag } from "../elements/prefixTag";
import createExcludedCourseTag from "../elements/excludedCourseTag";

const closeConfigMenuBtn = document.querySelector("#closeConfigMenuBtn");
const curriculumInput = document.querySelector("#curriculumId");

const addPrefixBtn = document.querySelector("#addPrefixBtn");
const prefixStorage = document.querySelector("#prefixStorage");
const prefixInput = document.querySelector("#prefixInput");

const excludedCourseBtn = document.querySelector("#excludeCourseBtn");
const excludedCourseStorage = document.querySelector("#excludedCoursesStorage");
const excludedCourseInput = document.querySelector("#excludeCourseInput");

const additionalConstraints = document.querySelectorAll(
  ".additionalPrefixConstraint",
);

const objectiveContainers = document.querySelectorAll(".objective-container");
const setConfigurationBtn = document.querySelector("#setConfigurationBtn");

export default function getConfigMenu(context, callBackToClose) {
  function addSelectedPrefix() {
    const prefix = prefixInput.value;
    if (prefix === "") {
      return;
    }
    const prefixTag = createPrefixTag(prefix);
    prefixStorage.appendChild(prefixTag);
    prefixInput.value = "";
    context.addPrefix(prefix);
  }

  function excludeSelectedCourse() {
    const excludedCourse = excludedCourseInput.value;
    if (excludedCourse === "") {
      return;
    }
    const excludedCourseTag = createExcludedCourseTag(excludedCourse);
    excludedCourseStorage.appendChild(excludedCourseTag);
    excludedCourseInput.value = "";
    context.excludeCourse(excludedCourse);
  }

  function setConfiguration() {
    const curriculum = curriculumInput.value;
    const additionalConstraintInputs = document.querySelectorAll(
      ".additionalConstraintInput",
    );

    context.setCurriculum(curriculum);
    additionalConstraintInputs.forEach((input) => {
      context.addAdditionalConstraint(input.name, input.value);
    });

    callBackToClose();
    console.log(context);
  }

  function openSubMenu() {
    console.log("Opening Config Menu");
    // included prefixes
    addPrefixBtn.onclick = addSelectedPrefix;
    // excluded courses
    excludedCourseBtn.onclick = excludeSelectedCourse;
    console.log(additionalConstraints);

    additionalConstraints.forEach((constr) => {
      const checkBox = constr.querySelector(".additionalConstraintCheck");
      checkBox.onclick = () => {
        constr.querySelectorAll("span").forEach((span) => {
          if (checkBox.checked) {
            span.style.color = "var(--white)";
          } else {
            span.style.color = "var(--darkgrey3)";
          }
        });

        const numberInput = constr.querySelector("input[type=number]");
        if (checkBox.checked) {
          numberInput.style.borderColor = "var(--white)";
          numberInput.style.color = "var(--white)";
          numberInput.value = 0;
          numberInput.disabled = false;
        } else {
          numberInput.style.borderColor = "var(--darkgrey3)";
          numberInput.value = "";
          numberInput.disabled = true;
        }
      };
    });

    objectiveContainers.forEach((cont) => {
      cont.onclick = () => {
        const objective = cont.textContent;
        context.setObjective(objective);
        const radioinput = cont.querySelector('[name="objectiveoption"]');
        radioinput.checked = true;
      };
    });

    closeConfigMenuBtn.onclick = callBackToClose;
    setConfigurationBtn.onclick = setConfiguration;
  }

  openSubMenu();
}
