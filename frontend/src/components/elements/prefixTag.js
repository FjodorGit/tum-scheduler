const prefixTagTemplete = document.querySelector("#choosenTag");
const additionalPrefixConstraintTemplete = document.querySelector(
  "#additionalPrefixConstraint",
);
const additionalConstraintsContainer = document.querySelector(
  "#additionalConstraintsContainer",
);

function createPrefixTag(prefix, context) {
  const prefixTagFragment = prefixTagTemplete.content.cloneNode(true);
  const prefixTag = prefixTagFragment.children[0];
  prefixTag.id = "prefixTag" + prefix;
  prefixTag.children[0].textContent = prefix;

  const additionalPrefixConstraintFragment =
    additionalPrefixConstraintTemplete.content.cloneNode(true);
  const additionalPrefixConstraint =
    additionalPrefixConstraintFragment.children[0];
  additionalPrefixConstraint.id = "additionalPrefixConstraint" + prefix;
  context.addAdditionalConstraintInput(prefix + "constraint");
  additionalPrefixConstraint.children[1].id = prefix + "constraint";
  additionalPrefixConstraint.children[2].textContent =
    "courses with prefix '" + prefix + "'.";
  function removePrefixTag() {
    const tagParent = document.getElementById(prefixTag.id);
    const additionalConstraintParent = document.getElementById(
      additionalPrefixConstraint.id,
    );
    tagParent.remove();
    additionalConstraintParent.remove();
  }
  prefixTag.children[1].onclick = removePrefixTag;
  additionalConstraintsContainer.appendChild(additionalPrefixConstraint);

  return prefixTag;
}

export { createPrefixTag };
