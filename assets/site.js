// Shared site logic: lesson list, top bar, progress, pager and the memory stepper.

const LESSONS = [
  { id: "rb1-basics", num: "0.1", title: "Variables, types, functions, loops", blurb: "Rust Book ch. 3: every type is a size in bytes.", mins: 20, group: "Rust Book essentials (start here)" },
  { id: "rb2-ownership", num: "0.2", title: "Ownership, borrowing, slices", blurb: "Rust Book ch. 4: the most important chapter, explained with stack and heap.", mins: 30, group: "Rust Book essentials (start here)" },
  { id: "rb3-collections", num: "0.3", title: "Vec, String, Box and Drop", blurb: "Rust Book ch. 8 and 15: the heap types and how cleanup works.", mins: 25, group: "Rust Book essentials (start here)" },
  { id: "rb4-threads", num: "0.4", title: "Threads, Arc and Mutex", blurb: "Rust Book ch. 16: just enough to understand data races.", mins: 20, group: "Rust Book essentials (start here)" },
  { id: "rb5-unsafe", num: "0.5", title: "Unsafe Rust basics", blurb: "Rust Book ch. 20.1: raw pointers, static mut, and what unsafe really allows.", mins: 20, group: "Rust Book essentials (start here)" },
  { id: "01-virtual-memory", num: "1.1", title: "Virtual memory and pages", blurb: "Why every program thinks it owns the whole machine.", mins: 15, group: "Lesson 1: Memory" },
  { id: "02-process-layout", num: "1.2", title: "The map of a Rust process", blurb: "Text, data, BSS, heap, stack, and who may read, write or run each.", mins: 15, group: "Lesson 1: Memory" },
  { id: "03-stack", num: "1.3", title: "The stack and stack frames", blurb: "Step through main() calling foo() one line at a time.", mins: 20, group: "Lesson 1: Memory" },
  { id: "04-heap", num: "1.4", title: "The heap: Box and Vec", blurb: "Pointers, length, capacity, and what happens when a Vec grows.", mins: 20, group: "Lesson 1: Memory" },
  { id: "05-static", num: "1.5", title: "Static memory and strings", blurb: "Statics, string literals, and &str vs String.", mins: 15, group: "Lesson 1: Memory" },
  { id: "06-on-your-machine", num: "1.6", title: "See it on your own machine", blurb: "Extra practice, no new theory: run the code and see real addresses, sizes and a stack overflow.", mins: 20, group: "Lesson 1: Memory" },
  { id: "07-memory-bugs", num: "2.1", title: "What a memory bug really is", blurb: "Undefined behaviour, spatial vs temporal safety, the formal definitions.", mins: 15, group: "Lesson 2: Memory bugs" },
  { id: "08-use-after-free", num: "2.2", title: "Use-after-free", blurb: "Free memory, keep the pointer, read someone else's data.", mins: 15, group: "Lesson 2: Memory bugs" },
  { id: "09-buffer-overflow", num: "2.3", title: "Buffer overflow", blurb: "Write one byte too far and watch a neighbour change.", mins: 15, group: "Lesson 2: Memory bugs" },
  { id: "10-data-race", num: "2.4", title: "Data race", blurb: "Two threads, one counter, lost updates, and how to fix it.", mins: 20, group: "Lesson 2: Memory bugs" },
  { id: "12-mastery", num: "★", title: "Mastery test: Lessons 1 and 2", blurb: "A \"where does it live?\" game and a scored quiz. Get 90% and you own this module.", mins: 20, group: "Prove it" },
  { id: "11-next", num: "✦", title: "Wrap-up and a peek at Lesson 3", blurb: "Optional: a checklist for Lessons 1 and 2, and how safe Rust stops each bug.", mins: 10, group: "Optional: after Lesson 2" },
];

const STORE = "rmm-done";
function getDone() {
  try { return new Set(JSON.parse(localStorage.getItem(STORE) || "[]")); } catch { return new Set(); }
}
function setDone(set) {
  try { localStorage.setItem(STORE, JSON.stringify([...set])); } catch { /* storage blocked: progress just won't persist */ }
}

const inLessons = location.pathname.includes("/lessons/");
const root = inLessons ? "../" : "./";
const here = location.pathname.split("/").pop().replace(".html", "");

function renderTop() {
  const done = getDone();
  const pct = Math.round((done.size / LESSONS.length) * 100);
  const top = document.createElement("header");
  top.className = "top";
  top.innerHTML = `
    <a class="brand" href="${root}index.html">Rust <span>Memory</span></a>
    <div class="bar"><i style="width:${pct}%"></i></div>
    <span class="count">${done.size}/${LESSONS.length} done</span>`;
  document.body.prepend(top);
}

function renderLessonChrome() {
  const i = LESSONS.findIndex((l) => l.id === here);
  if (i < 0) return;
  const main = document.querySelector("main");
  const done = getDone();
  const finish = document.createElement("div");
  finish.className = "finish";
  const label = () => (getDone().has(here) ? "Done ✓ (click to undo)" : "Mark lesson as done");
  finish.innerHTML = `<span>Finished reading, ran the code, and answered the questions?</span><button class="primary">${label()}</button>`;
  finish.querySelector("button").onclick = (e) => {
    const s = getDone();
    s.has(here) ? s.delete(here) : s.add(here);
    setDone(s);
    e.target.textContent = label();
    document.querySelector(".top").remove();
    renderTop();
  };
  main.appendChild(finish);

  const prev = LESSONS[i - 1], next = LESSONS[i + 1];
  const pager = document.createElement("nav");
  pager.className = "pager";
  pager.innerHTML =
    (prev ? `<a href="${prev.id}.html"><small>← Previous</small>${prev.title}</a>` : `<a href="../index.html"><small>←</small>All lessons</a>`) +
    (next ? `<a class="next" href="${next.id}.html"><small>Next →</small>${next.title}</a>` : `<a class="next" href="../index.html"><small>→</small>All lessons</a>`);
  main.appendChild(pager);
  void done;
}

function renderIndex() {
  const el = document.getElementById("lesson-list");
  if (!el) return;
  const done = getDone();
  let html = "", group = "";
  LESSONS.forEach((l) => {
    if (l.group !== group) { group = l.group; html += `<div class="group">${group}</div>`; }
    html += `<a class="card ${done.has(l.id) ? "done" : ""}" href="lessons/${l.id}.html">
      <div class="num">${done.has(l.id) ? "✓" : l.num}</div>
      <div><h3>${l.title}</h3><p>${l.blurb}</p></div>
      <div class="mins">${l.mins} min</div></a>`;
  });
  el.innerHTML = html;
}

// Stepper. Each step is a full snapshot:
// { line: 3 | [3,4], cols: { Stack: [box], Heap: [box], ... }, note: "html" }
// box = { t: "title", r: [[name, value, flag]], f: flag }   flag: "new" | "freed" | "bad"
function stepper(el, cfg) {
  const lines = cfg.code.replace(/^\n/, "").split("\n");
  let k = 0;
  el.classList.add("stepper");
  el.innerHTML = `
    <div class="head"><span>${cfg.title || "Step through it"}</span><span class="mono pos"></span></div>
    <div class="body"><pre class="code"></pre><div class="mem"></div></div>
    <div class="say"></div>
    <div class="ctl"><button class="prev">◀ Back</button><div class="dots"></div><button class="next primary">Next ▶</button></div>`;
  const code = el.querySelector(".code"), mem = el.querySelector(".mem"), say = el.querySelector(".say");
  const dots = el.querySelector(".dots");
  code.innerHTML = lines.map((t, n) => `<div><span class="n">${n + 1}</span>${esc(t)}</div>`).join("");
  dots.innerHTML = cfg.steps.map(() => "<i></i>").join("");
  dots.querySelectorAll("i").forEach((d, n) => (d.onclick = () => go(n)));

  const colClass = (name) => {
    const s = name.toLowerCase();
    return s.includes("stack") ? "c-stack" : s.includes("heap") ? "c-heap" : s.includes("static") ? "c-static" : "c-other";
  };
  function go(n) {
    k = Math.max(0, Math.min(cfg.steps.length - 1, n));
    const s = cfg.steps[k];
    const on = new Set([].concat(s.line ?? []));
    code.querySelectorAll("div").forEach((d, n) => d.classList.toggle("on", on.has(n + 1)));
    const cols = cfg.cols || Object.keys(s.cols);
    mem.innerHTML = cols.map((c) => `
      <div class="col ${colClass(c)}"><h4>${c}</h4>${(s.cols[c] || []).map((b) => `
        <div class="box ${b.f || ""}"><div class="bt">${b.t}</div>${(b.r || []).map((r) =>
          `<div class="row ${r[2] || ""}"><span>${r[0]}</span><span>${r[1]}</span></div>`).join("")}</div>`).join("") ||
        '<div class="addr">(empty)</div>'}</div>`).join("");
    say.innerHTML = s.note;
    el.querySelector(".pos").textContent = `${k + 1} / ${cfg.steps.length}`;
    el.querySelector(".prev").disabled = k === 0;
    el.querySelector(".next").disabled = k === cfg.steps.length - 1;
    dots.querySelectorAll("i").forEach((d, n) => d.classList.toggle("on", n === k));
  }
  el.querySelector(".prev").onclick = () => go(k - 1);
  el.querySelector(".next").onclick = () => go(k + 1);
  el.tabIndex = 0;
  el.addEventListener("keydown", (e) => {
    if (e.key === "ArrowRight") { go(k + 1); e.preventDefault(); }
    if (e.key === "ArrowLeft") { go(k - 1); e.preventDefault(); }
  });
  go(0);
}
function esc(s) { return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;"); }

renderTop();
renderLessonChrome();
renderIndex();
