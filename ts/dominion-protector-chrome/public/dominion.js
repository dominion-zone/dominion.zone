var N = function(e, t, n, r) {
  if (n === "a" && !r) throw new TypeError("Private accessor was defined without a getter");
  if (typeof t == "function" ? e !== t || !r : !t.has(e)) throw new TypeError("Cannot read private member from an object whose class did not declare it");
  return n === "m" ? r : n === "a" ? r.call(e) : r ? r.value : t.get(e);
}, y = function(e, t, n, r, a) {
  if (r === "m") throw new TypeError("Private method is not writable");
  if (r === "a" && !a) throw new TypeError("Private accessor was defined without a setter");
  if (typeof t == "function" ? e !== t || !a : !t.has(e)) throw new TypeError("Cannot write private member to an object whose class did not declare it");
  return r === "a" ? a.call(e, n) : a ? a.value = n : t.set(e, n), n;
}, M;
let w;
const f = /* @__PURE__ */ new Set();
function S(e) {
  p = void 0, f.add(e);
}
function h(e) {
  p = void 0, f.delete(e);
}
const g = {};
function T() {
  if (w || (w = Object.freeze({ register: E, get: A, on: m }), typeof window > "u"))
    return w;
  const e = Object.freeze({ register: E });
  try {
    window.addEventListener("wallet-standard:register-wallet", ({ detail: t }) => t(e));
  } catch (t) {
    console.error(`wallet-standard:register-wallet event listener could not be added
`, t);
  }
  try {
    window.dispatchEvent(new D(e));
  } catch (t) {
    console.error(`wallet-standard:app-ready event could not be dispatched
`, t);
  }
  return w;
}
function E(...e) {
  var t;
  return e = e.filter((n) => !f.has(n)), e.length ? (e.forEach((n) => S(n)), (t = g.register) == null || t.forEach((n) => I(() => n(...e))), function() {
    var r;
    e.forEach((a) => h(a)), (r = g.unregister) == null || r.forEach((a) => I(() => a(...e)));
  }) : () => {
  };
}
let p;
function A() {
  return p || (p = [...f]), p;
}
function m(e, t) {
  var n;
  return (n = g[e]) != null && n.push(t) || (g[e] = [t]), function() {
    var a;
    g[e] = (a = g[e]) == null ? void 0 : a.filter((i) => t !== i);
  };
}
function I(e) {
  try {
    e();
  } catch (t) {
    console.error(t);
  }
}
class D extends Event {
  get detail() {
    return N(this, M, "f");
  }
  get type() {
    return "wallet-standard:app-ready";
  }
  constructor(t) {
    super("wallet-standard:app-ready", {
      bubbles: !1,
      cancelable: !1,
      composed: !1
    }), M.set(this, void 0), y(this, M, t, "f");
  }
  /** @deprecated */
  preventDefault() {
    throw new Error("preventDefault cannot be called");
  }
  /** @deprecated */
  stopImmediatePropagation() {
    throw new Error("stopImmediatePropagation cannot be called");
  }
  /** @deprecated */
  stopPropagation() {
    throw new Error("stopPropagation cannot be called");
  }
}
M = /* @__PURE__ */ new WeakMap();
var b = function(e, t, n, r) {
  if (n === "a" && !r) throw new TypeError("Private accessor was defined without a getter");
  if (typeof t == "function" ? e !== t || !r : !t.has(e)) throw new TypeError("Cannot read private member from an object whose class did not declare it");
  return n === "m" ? r : n === "a" ? r.call(e) : r ? r.value : t.get(e);
}, x = function(e, t, n, r, a) {
  if (r === "m") throw new TypeError("Private method is not writable");
  if (r === "a" && !a) throw new TypeError("Private accessor was defined without a setter");
  if (typeof t == "function" ? e !== t || !a : !t.has(e)) throw new TypeError("Cannot write private member to an object whose class did not declare it");
  return r === "a" ? a.call(e, n) : a ? a.value = n : t.set(e, n), n;
}, j;
function v(e) {
  const t = ({ register: n }) => n(e);
  try {
    window.dispatchEvent(new C(t));
  } catch (n) {
    console.error(`wallet-standard:register-wallet event could not be dispatched
`, n);
  }
  try {
    window.addEventListener("wallet-standard:app-ready", ({ detail: n }) => t(n));
  } catch (n) {
    console.error(`wallet-standard:app-ready event listener could not be added
`, n);
  }
}
class C extends Event {
  get detail() {
    return b(this, j, "f");
  }
  get type() {
    return "wallet-standard:register-wallet";
  }
  constructor(t) {
    super("wallet-standard:register-wallet", {
      bubbles: !1,
      cancelable: !1,
      composed: !1
    }), j.set(this, void 0), x(this, j, t, "f");
  }
  /** @deprecated */
  preventDefault() {
    throw new Error("preventDefault cannot be called");
  }
  /** @deprecated */
  stopImmediatePropagation() {
    throw new Error("stopImmediatePropagation cannot be called");
  }
  /** @deprecated */
  stopPropagation() {
    throw new Error("stopPropagation cannot be called");
  }
}
j = /* @__PURE__ */ new WeakMap();
const z = [
  "standard:connect",
  "standard:events"
];
function O(e, t = []) {
  return [...z, ...t].every(
    (n) => n in e.features
  );
}
const P = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNjQiIGhlaWdodD0iNjQiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PHJlY3Qgd2lkdGg9IjY0IiBoZWlnaHQ9IjY0IiByeD0iMjQiIGZpbGw9InVybCgjcGFpbnQwX3JhZGlhbF8zMDVfMTI1MTYpIi8+PHBhdGggZD0iTTUxLjUgNDMuNmMtMy45IDAtNy42LTMuOS05LjUtNi40LTEuOSAyLjUtNS42IDYuNC05LjUgNi40LTQgMC03LjctMy45LTkuNS02LjQtMS44IDIuNS01LjUgNi40LTkuNSA2LjQtLjggMC0xLjUtLjYtMS41LTEuNSAwLS44LjctMS41IDEuNS0xLjUgMy4yIDAgNy4xLTUuMSA4LjItNi45LjMtLjQuOC0uNyAxLjMtLjdzMSAuMiAxLjMuN2MxLjEgMS44IDUgNi45IDguMiA2LjkgMy4xIDAgNy4xLTUuMSA4LjItNi45LjMtLjQuOC0uNyAxLjMtLjdzMSAuMiAxLjIuN2MxLjEgMS44IDUgNi45IDguMiA2LjkuOSAwIDEuNi43IDEuNiAxLjUgMCAuOS0uNiAxLjUtMS41IDEuNXoiIGZpbGw9IiNmZmYiLz48cGF0aCBkPSJNNTEuNSA1Mi4zYy0zLjkgMC03LjYtMy45LTkuNS02LjQtMS45IDIuNS01LjYgNi40LTkuNSA2LjQtNCAwLTcuNy0zLjktOS41LTYuNC0xLjggMi41LTUuNSA2LjQtOS41IDYuNC0uOCAwLTEuNS0uNi0xLjUtMS41IDAtLjguNy0xLjUgMS41LTEuNSAzLjIgMCA3LjEtNS4xIDguMi02LjkuMy0uNC44LS43IDEuMy0uN3MxIC4zIDEuMy43YzEuMSAxLjggNSA2LjkgOC4yIDYuOSAzLjEgMCA3LjEtNS4xIDguMi02LjkuMy0uNC44LS43IDEuMy0uN3MxIC4zIDEuMi43YzEuMSAxLjggNSA2LjkgOC4yIDYuOS45IDAgMS42LjcgMS42IDEuNSAwIC45LS42IDEuNS0xLjUgMS41ek0xNC42IDM2LjdjLS44IDAtMS40LS41LTEuNi0xLjNsLS4zLTMuNmMwLTEwLjkgOC45LTE5LjggMTkuOC0xOS44IDExIDAgMTkuOCA4LjkgMTkuOCAxOS44bC0uMyAzLjZjLS4xLjgtLjkgMS40LTEuNyAxLjItLjktLjEtMS41LS45LTEuMy0xLjhsLjMtM2MwLTkuMi03LjUtMTYuOC0xNi44LTE2LjgtOS4yIDAtMTYuNyA3LjUtMTYuNyAxNi44bC4yIDMuMWMuMi44LS4zIDEuNi0xLjEgMS44aC0uM3oiIGZpbGw9IiNmZmYiLz48ZGVmcz48cmFkaWFsR3JhZGllbnQgaWQ9InBhaW50MF9yYWRpYWxfMzA1XzEyNTE2IiBjeD0iMCIgY3k9IjAiIHI9IjEiIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIiBncmFkaWVudFRyYW5zZm9ybT0ibWF0cml4KDUyLjc1ODAzIDUxLjM1OCAtNTEuNDM5NDcgNTIuODQxNzIgMCA3LjQwNykiPjxzdG9wIHN0b3AtY29sb3I9IiMwMDU4REQiLz48c3RvcCBvZmZzZXQ9IjEiIHN0b3AtY29sb3I9IiM2N0M4RkYiLz48L3JhZGlhbEdyYWRpZW50PjwvZGVmcz48L3N2Zz4=";
function W(e) {
  if (!O(e) || !e.chains.some((o) => o.split(":")[0] === "sui"))
    return null;
  const t = `DP:${e.id || e.name}`, n = `DP: ${e.name}`, r = ({
    tx: o,
    account: l,
    chain: c,
    signal: s
  }) => new Promise((L, d) => {
    window.addEventListener("message", (u) => {
      if (console.log("Dominion event", u), !(u.source !== window || !u.data || u.data.source !== "content-script")) {
        if (s != null && s.aborted)
          return d(s.reason);
        if (!u.data.approve)
          return d("Dominion protection has canceled the transaction");
        L(u.data.approve);
      }
    }), window.postMessage({
      checkTransaction: {
        transaction: o,
        account: l,
        chain: c
      },
      source: "dominion-page-script"
    });
  }), a = {
    ...e.features
  };
  if (e.features["sui:signTransaction"]) {
    const o = e.features;
    if (o["sui:signTransaction"].version !== "2.0.0")
      throw new Error(
        "Unsupported version of the sui:signTransaction feature. Expected version 2.0.0."
      );
    const l = async ({
      transaction: c,
      account: s,
      chain: L,
      signal: d
    }) => {
      const u = await c.toJSON();
      return await r({ tx: u, account: s, chain: L, signal: d }), await o["sui:signTransaction"].signTransaction({
        transaction: { ...c, toJSON: async () => u },
        account: s,
        chain: L,
        signal: d
      });
    };
    a["sui:signTransaction"] = {
      version: "2.0.0",
      signTransaction: l
    };
  }
  if (e.features["sui:signAndExecuteTransaction"]) {
    const o = e.features;
    if (o["sui:signAndExecuteTransaction"].version !== "2.0.0")
      throw new Error(
        "Unsupported version of the sui:signAndExecuteTransaction feature. Expected version 2.0.0."
      );
    const l = async ({
      transaction: c,
      account: s,
      chain: L,
      signal: d
    }) => {
      const u = await c.toJSON();
      return await r({ tx: u, account: s, chain: L, signal: d }), await o["sui:signAndExecuteTransaction"].signAndExecuteTransaction({
        transaction: { ...c, toJSON: async () => u },
        account: s,
        chain: L,
        signal: d
      });
    };
    a["sui:signAndExecuteTransaction"] = {
      version: "2.0.0",
      signAndExecuteTransaction: l
    };
  }
  const i = Object.freeze({
    version: "1.0.0",
    id: t,
    name: n,
    icon: P,
    chains: e.chains,
    features: a,
    accounts: []
  });
  return v(i), i;
}
async function U() {
  const e = /* @__PURE__ */ new Set(), t = (...r) => (r = r.filter(
    (a) => {
      var i;
      return !e.has(a) && !((i = a == null ? void 0 : a.id) != null && i.startsWith("DP:"));
    }
  ), r.length ? (r.map((a) => (e.add(a), W(a))).filter((a) => a), window.postMessage({
    registeredWallets: r.map(
      ({ id: a, name: i, icon: o, chains: l, features: c, accounts: s }) => ({
        id: a,
        name: i,
        icon: o,
        chains: l,
        features: Object.keys(c),
        accounts: s
      })
    ),
    source: "dominion-page-script"
  }), function() {
    r.forEach((i) => e.delete(i)), window.postMessage({
      unregisteredWallets: r.map(({ id: i, name: o }) => ({
        id: i,
        name: o
      })),
      source: "dominion-page-script"
    });
  }) : () => {
  }), n = T();
  t(...n.get()), n.on("register", t);
}
(async function() {
  U();
})();
