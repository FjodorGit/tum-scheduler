import setHeader from "../components/menus/header";
import setYearView from "../components/views/yearview";
import setWeekView from "../components/views/weekview";

const weekComponent = document.querySelector(".weekview");

export default function setViews(component, context, store, datepickerContext) {
  function initView(component) {
    context.setComponent(component);
    setHeader(context, component);
    setWeekView(context, store, datepickerContext);
    weekComponent.classList.remove("hide-view");
  }

  document.title = context.getMonthName();
  initView(component);
}

