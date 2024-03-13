export default function setHeader(context, component, store) {
  const dateTimeTitle = document.querySelector(".datetime-content--title");
  const header = document.querySelector(".header");
  const datetimeWrapper = document.querySelector(".h-col-2");
  const datetimeContent = document.querySelector(".datetime-content");
  const prevnext = document.querySelector(".prev-next");

  const configHeader = (borderstyle, componentTitle) => {
    header.style.borderBottom = borderstyle;
    dateTimeTitle.textContent = componentTitle;
    datetimeWrapper.classList.remove("datetime-inactive");
    datetimeWrapper.style.paddingRight = "0";
    datetimeContent.removeAttribute("style");
    prevnext.removeAttribute("style");
  };
}
