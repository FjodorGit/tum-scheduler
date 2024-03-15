const prefixTagTemplete = document.querySelector("#choosenTag");
const additionalPrefixConstraintTemplete = document.querySelector(
  "#additionalPrefixConstraintTemplate",
);
const additionalConstraintsContainer = document.querySelector(
  "#additionalConstraintsContainer",
);

function addReactivityToConstraint(constr, prefix) {
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
    numberInput.name = prefix;
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
}

function createPrefixTag(prefix) {
  const prefixTagFragment = prefixTagTemplete.content.cloneNode(true);
  const prefixTag = prefixTagFragment.children[0];
  prefixTag.id = "prefixTag" + prefix;
  prefixTag.children[0].textContent = prefix;

  const additionalPrefixConstraintFragment =
    additionalPrefixConstraintTemplete.content.cloneNode(true);
  const additionalPrefixConstraint =
    additionalPrefixConstraintFragment.children[0];

  addReactivityToConstraint(additionalPrefixConstraint, prefix);

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
