const indexToWeekday = {
  0: "Monday",
  1: "Tuesday",
  2: "Wednesday",
  3: "Thursday",
  4: "Friday",
};

class Configuration {
  constructor() {
    this.num_of_blockers = 0;
    this.semester = "";
    this.curriculum = "";
    this.selectedPrefixes = [];
    this.excludedCourses = [];
    this.additionalConstraints = {};
    this.objective = "noobjective";
    this.blockers = {};
  }

  setSemester(semester) {
    this.semester = semester;
  }

  setCurriculum(curriculum) {
    this.curriculum = curriculum;
  }

  addPrefix(prefix) {
    this.selectedPrefixes.push(prefix);
  }

  excludeCourse(course) {
    this.excludedCourses.push(course);
  }

  addAdditionalConstraint(name, amount) {
    this.additionalConstraints[name] = amount;
  }

  setObjective(objective) {
    this.objective = objective;
  }

  addBlocker(col, from, until) {
    const weekday = indexToWeekday[col.getAttribute("data-column-index")];
    console.log(col.getAttribute("data-column-index"));
    console.log(weekday);
    const appointment = { weekday: weekday, from: from, until: until };
    const blockerId = "blocker" + this.num_of_blockers.toString();
    this.num_of_blockers += 1;
    this.blockers[blockerId] = appointment;
    return blockerId;
  }

  removeBlocker(blockerId) {
    delete this.blockers[blockerId];
  }

  as_json() {
    return JSON.stringify(this);
  }
}

const configuration = new Configuration();
export default configuration;
