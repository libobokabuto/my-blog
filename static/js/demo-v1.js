const links = [...document.querySelectorAll(".topnav a")];
const sections = [...document.querySelectorAll(".preview-section")];

const setActiveLink = () => {
  const current = sections.find((section) => {
    const rect = section.getBoundingClientRect();
    return rect.top <= 160 && rect.bottom >= 160;
  });

  links.forEach((link) => {
    const targetId = link.getAttribute("href")?.slice(1);
    link.classList.toggle("is-active", current?.id === targetId);
  });
};

document.addEventListener("scroll", setActiveLink, { passive: true });
window.addEventListener("load", setActiveLink);
