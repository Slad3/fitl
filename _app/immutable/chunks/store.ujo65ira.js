import{al as l,$ as f,am as y,Q as g,an as E,ao as W,ap as k,T as q,F as _,x as D,a7 as F,A as M,O as N}from"./runtime.CTT-Pcax.js";import{a as P}from"./misc.CIFqOFj7.js";function L(n){var t=y,r=g;l(null),f(null);try{return n()}finally{l(t),f(r)}}function j(n,t,r,e=r){n.addEventListener(t,()=>L(r));const a=n.__on_r;a?n.__on_r=()=>{a(),e()}:n.__on_r=e,P()}const z=new Set,A=new Set;function C(n,t,r,e){function a(s){if(e.capture||x.call(t,s),!s.cancelBubble)return L(()=>r.call(this,s))}return n.startsWith("pointer")||n.startsWith("touch")||n==="wheel"?W(()=>{t.addEventListener(n,a,e)}):t.addEventListener(n,a,e),a}function G(n,t,r,e,a){var s={capture:e,passive:a},u=C(n,t,r,s);(t===document.body||t===window||t===document)&&E(()=>{t.removeEventListener(n,u,s)})}function x(n){var w;var t=this,r=t.ownerDocument,e=n.type,a=((w=n.composedPath)==null?void 0:w.call(n))||[],s=a[0]||n.target,u=0,p=n.__root;if(p){var b=a.indexOf(p);if(b!==-1&&(t===document||t===window)){n.__root=t;return}var h=a.indexOf(t);if(h===-1)return;b<=h&&(u=b)}if(s=a[u]||n.target,s!==t){k(n,"currentTarget",{configurable:!0,get(){return s||r}});var T=y,B=g;l(null),f(null);try{for(var o,m=[];s!==null;){var v=s.assignedSlot||s.parentNode||s.host||null;try{var i=s["__"+e];if(i!==void 0&&!s.disabled)if(q(i)){var[O,...S]=i;O.apply(s,[n,...S])}else i.call(s,n)}catch(c){o?m.push(c):o=c}if(n.cancelBubble||v===t||v===null)break;s=v}if(o){for(let c of m)queueMicrotask(()=>{throw c});throw o}}finally{n.__root=t,delete n.currentTarget,l(T),f(B)}}}function H(n,t,r){if(n==null)return t(void 0),_;const e=D(()=>n.subscribe(t,r));return e.unsubscribe?()=>e.unsubscribe():e}let d=!1;function I(n,t,r){const e=r[t]??(r[t]={store:null,source:F(void 0),unsubscribe:_});if(e.store!==n)if(e.unsubscribe(),e.store=n??null,n==null)e.source.v=void 0,e.unsubscribe=_;else{var a=!0;e.unsubscribe=H(n,s=>{a?e.source.v=s:N(e.source,s)}),a=!1}return M(e.source)}function J(){const n={};return E(()=>{for(var t in n)n[t].unsubscribe()}),n}function Q(n){var t=d;try{return d=!1,[n(),d]}finally{d=t}}export{I as a,z as b,Q as c,G as e,x as h,j as l,A as r,J as s};
