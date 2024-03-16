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

export default function setupConfigMenu(configuration, callBackToClose) {
  function addSelectedPrefix() {
    const prefix = prefixInput.value;
    if (prefix === "") {
      return;
    }
    const prefixTag = createPrefixTag(prefix);
    prefixStorage.appendChild(prefixTag);
    prefixInput.value = "";
    configuration.addPrefix(prefix);
  }

  function excludeSelectedCourse() {
    const excludedCourse = excludedCourseInput.value;
    if (excludedCourse === "") {
      return;
    }
    const excludedCourseTag = createExcludedCourseTag(excludedCourse);
    excludedCourseStorage.appendChild(excludedCourseTag);
    excludedCourseInput.value = "";
    configuration.excludeCourse(excludedCourse);
  }

  function setConfiguration() {
    const curriculum = curriculumInput.value;
    const additionalConstraintInputs = document.querySelectorAll(
      ".additionalConstraintInput",
    );

    configuration.setCurriculum(curriculum);
    additionalConstraintInputs.forEach((input) => {
      if (!input.disabled) {
        configuration.addAdditionalConstraint(
          input.name,
          parseInt(input.value),
        );
      }
    });

    callBackToClose();
    console.log(configuration.as_json());
  }

  function openSubMenu() {
    // included prefixes
    addPrefixBtn.onclick = addSelectedPrefix;
    // excluded courses
    excludedCourseBtn.onclick = excludeSelectedCourse;

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
        const objective = cont.querySelector("input").value;
        configuration.setObjective(objective);
        const radioinput = cont.querySelector('[name="objectiveoption"]');
        radioinput.checked = true;
      };
    });

    closeConfigMenuBtn.onclick = callBackToClose;
    setConfigurationBtn.onclick = setConfiguration;
  }

  openSubMenu();
}
