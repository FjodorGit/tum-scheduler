const excludedCourseTagTemplate = document.querySelector("#choosenTag");

export default function createExcludedCourseTag(prefix) {
  const excludedCourseTagFragment =
    excludedCourseTagTemplate.content.cloneNode(true);
  const excludedCourseTag = excludedCourseTagFragment.children[0];
  excludedCourseTag.id = "excludedCourseTag" + prefix;
  excludedCourseTag.children[0].textContent = prefix;

  function removeExcludedCourseTag() {
    const tagParent = document.getElementById(excludedCourseTag.id);
    tagParent.remove();
  }
  excludedCourseTag.children[1].onclick = removeExcludedCourseTag;
  return excludedCourseTag;
}
