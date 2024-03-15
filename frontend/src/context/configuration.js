class Configuration {
  constructor() {
    this.curriculum = "";
    this.selectedPrefixes = [];
    this.excludedCourses = [];
    this.addAdditionalConstraints = {};
    this.objective = "noobjective";
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
    this.addAdditionalConstraints[name] = amount;
  }

  setObjective(objective) {
    this.objective = objective;
  }

  as_json() {
    return JSON.stringify(this);
  }
}

const configuration = new Configuration();
export default configuration;
