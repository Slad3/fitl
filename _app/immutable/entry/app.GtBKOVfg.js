const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["../nodes/0.BajxbEdp.js","../chunks/disclose-version.mL42o9Kk.js","../chunks/runtime.kV6Rt6fy.js","../chunks/legacy.6wHpMh_u.js","../chunks/store.DQeuOIQA.js","../chunks/misc.CIFqOFj7.js","../chunks/if.DTfWMcAP.js","../chunks/attributes.np55ldqT.js","../chunks/lifecycle.BrNh3NMC.js","../chunks/stores.D10imKGh.js","../chunks/entry.BOgH0KY1.js","../chunks/paths.ClMYDVb1.js","../chunks/index-client.Uwi0sjav.js","../assets/0.CoLj2480.css","../assets/app.CzrWal1D.css","../nodes/1.Cq-kP5IR.js","../chunks/render.Certklrj.js","../nodes/2.DlJ5xpEM.js","../chunks/FoodExample.D99GtVb4.js","../chunks/DataTable.qF5KbdkX.js","../chunks/props.DUKzf9R5.js","../assets/DataTable.liNzSQjp.css","../assets/FoodExample.DFezeqDs.css","../nodes/3.CA8vKJ9g.js","../nodes/4.D6PCfDOu.js","../nodes/5.CgJuLLHx.js","../assets/5.BykKWNw8.css","../assets/Help.CJGysf7S.css","../nodes/6.oVqZMxwT.js"])))=>i.map(i=>d[i]);
var M=i=>{throw TypeError(i)};var W=(i,u,m)=>u.has(i)||M("Cannot "+m);var h=(i,u,m)=>(W(i,u,"read from private field"),m?m.call(i):u.get(i)),V=(i,u,m)=>u.has(i)?M("Cannot add the same private member more than once"):u instanceof WeakSet?u.add(i):u.set(i,m),S=(i,u,m,O)=>(W(i,u,"write to private field"),O?O.call(i,m):u.set(i,m),m);import{h as G,c as nt,b as it,E as ct,a as lt,f as ut,p as mt,aC as pt,az as ht,q as dt,al as ft,S as _t,x as E,M as D,aD as vt,am as gt,a4 as Et,g as yt,u as $t,n as Rt,aE as bt,i as x,j as wt,aF as k,m as Pt,k as Ot,t as Lt,l as At,z as q}from"../chunks/runtime.kV6Rt6fy.js";import{h as Dt,m as xt,u as It,s as Tt}from"../chunks/render.Certklrj.js";import{c as C,a as b,t as H,d as Vt}from"../chunks/disclose-version.mL42o9Kk.js";import{i as j}from"../chunks/if.DTfWMcAP.js";import{p as z,a as St}from"../chunks/props.DUKzf9R5.js";import{o as kt}from"../chunks/index-client.Uwi0sjav.js";let J,K,N,Q,X,Y,qt=(async()=>{var g,f;function i(s,t,o){G&&nt();var l=s,a,r;it(()=>{a!==(a=t())&&(r&&(mt(r),r=null),a&&(r=lt(()=>o(l,a))))},ct),G&&(l=ut)}function u(s,t){return s===t||(s==null?void 0:s[_t])===t}function m(s={},t,o,l){return pt(()=>{var a,r;return ht(()=>{a=r,r=[],dt(()=>{s!==o(...r)&&(t(s,...r),a&&u(o(...a),s)&&t(null,...a))})}),()=>{ft(()=>{r&&u(o(...r),s)&&t(null,...r)})}}),s}function O(s){return class extends Z{constructor(t){super({component:s,...t})}}}class Z{constructor(t){V(this,g);V(this,f);var r;var o=new Map,l=(e,n)=>{var c=Et(n);return o.set(e,c),c};const a=new Proxy({...t.props||{},$$events:{}},{get(e,n){return E(o.get(n)??l(n,Reflect.get(e,n)))},has(e,n){return E(o.get(n)??l(n,Reflect.get(e,n))),Reflect.has(e,n)},set(e,n,c){return D(o.get(n)??l(n,c),c),Reflect.set(e,n,c)}});S(this,f,(t.hydrate?Dt:xt)(t.component,{target:t.target,anchor:t.anchor,props:a,context:t.context,intro:t.intro??!1,recover:t.recover})),(!((r=t==null?void 0:t.props)!=null&&r.$$host)||t.sync===!1)&&vt(),S(this,g,a.$$events);for(const e of Object.keys(h(this,f)))e==="$set"||e==="$destroy"||e==="$on"||gt(this,e,{get(){return h(this,f)[e]},set(n){h(this,f)[e]=n},enumerable:!0});h(this,f).$set=e=>{Object.assign(a,e)},h(this,f).$destroy=()=>{It(h(this,f))}}$set(t){h(this,f).$set(t)}$on(t,o){h(this,g)[t]=h(this,g)[t]||[];const l=(...a)=>o.call(this,...a);return h(this,g)[t].push(l),()=>{h(this,g)[t]=h(this,g)[t].filter(a=>a!==l)}}$destroy(){h(this,f).$destroy()}}g=new WeakMap,f=new WeakMap;let U,B,I,y;U="modulepreload",B=function(s,t){return new URL(s,t).href},I={},y=function(s,t,o){let l=Promise.resolve();if(t&&t.length>0){const r=document.getElementsByTagName("link"),e=document.querySelector("meta[property=csp-nonce]"),n=(e==null?void 0:e.nonce)||(e==null?void 0:e.getAttribute("nonce"));l=Promise.allSettled(t.map(c=>{if(c=B(c,o),c in I)return;I[c]=!0;const $=c.endsWith(".css"),L=$?'[rel="stylesheet"]':"";if(o)for(let d=r.length-1;d>=0;d--){const p=r[d];if(p.href===c&&(!$||p.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${c}"]${L}`))return;const v=document.createElement("link");if(v.rel=$?"stylesheet":U,$||(v.as="script"),v.crossOrigin="",v.href=c,n&&v.setAttribute("nonce",n),document.head.appendChild(v),$)return new Promise((d,p)=>{v.addEventListener("load",d),v.addEventListener("error",()=>p(new Error(`Unable to preload CSS for ${c}`)))})}))}function a(r){const e=new Event("vite:preloadError",{cancelable:!0});if(e.payload=r,window.dispatchEvent(e),!e.defaultPrevented)throw r}return l.then(r=>{for(const e of r||[])e.status==="rejected"&&a(e.reason);return s().catch(a)})},N={};var tt=H('<div id="svelte-announcer" aria-live="assertive" aria-atomic="true" style="position: absolute; left: 0; top: 0; clip: rect(0 0 0 0); clip-path: inset(50%); overflow: hidden; white-space: nowrap; width: 1px; height: 1px"><!></div>'),et=H("<!> <!>",1);function rt(s,t){yt(t,!0);let o=z(t,"components",23,()=>[]),l=z(t,"data_0",3,null),a=z(t,"data_1",3,null);$t(()=>t.stores.page.set(t.page)),Rt(()=>{t.stores,t.page,t.constructors,o(),t.form,l(),a(),t.stores.page.notify()});let r=k(!1),e=k(!1),n=k(null);kt(()=>{const d=t.stores.page.subscribe(()=>{E(r)&&(D(e,!0),bt().then(()=>{D(n,St(document.title||"untitled page"))}))});return D(r,!0),d});const c=q(()=>t.constructors[1]);var $=et(),L=x($);j(L,()=>t.constructors[1],d=>{var p=C();const w=q(()=>t.constructors[0]);var P=x(p);i(P,()=>E(w),(R,T)=>{m(T(R,{get data(){return l()},get form(){return t.form},children:(_,Ct)=>{var F=C(),st=x(F);i(st,()=>E(c),(ot,at)=>{m(at(ot,{get data(){return a()},get form(){return t.form}}),A=>o()[1]=A,()=>{var A;return(A=o())==null?void 0:A[1]})}),b(_,F)},$$slots:{default:!0}}),_=>o()[0]=_,()=>{var _;return(_=o())==null?void 0:_[0]})}),b(d,p)},d=>{var p=C();const w=q(()=>t.constructors[0]);var P=x(p);i(P,()=>E(w),(R,T)=>{m(T(R,{get data(){return l()},get form(){return t.form}}),_=>o()[0]=_,()=>{var _;return(_=o())==null?void 0:_[0]})}),b(d,p)});var v=Pt(L,2);j(v,()=>E(r),d=>{var p=tt(),w=Ot(p);j(w,()=>E(e),P=>{var R=Vt();Lt(()=>Tt(R,E(n))),b(P,R)}),At(p),b(d,p)}),b(s,$),wt()}X=O(rt),Q=[()=>y(()=>import("../nodes/0.BajxbEdp.js"),__vite__mapDeps([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14]),import.meta.url),()=>y(()=>import("../nodes/1.Cq-kP5IR.js"),__vite__mapDeps([15,1,2,3,16,4,5,8,9,10,11]),import.meta.url),()=>y(()=>import("../nodes/2.DlJ5xpEM.js"),__vite__mapDeps([17,1,2,3,7,5,11,18,16,4,19,8,20,21,12,22]),import.meta.url),()=>y(()=>import("../nodes/3.CA8vKJ9g.js"),__vite__mapDeps([23,1,2,3,7,5]),import.meta.url),()=>y(()=>import("../nodes/4.D6PCfDOu.js"),__vite__mapDeps([24,1,2,3,18,16,4,5,7,19,8,20,21,12,22]),import.meta.url),()=>y(()=>import("../nodes/5.CgJuLLHx.js"),__vite__mapDeps([25,1,2,3,16,4,5,7,19,8,20,21,12,26,14,27]),import.meta.url),()=>y(()=>import("../nodes/6.oVqZMxwT.js"),__vite__mapDeps([28,1,2,3,27]),import.meta.url)],Y=[],J={"/":[2],"/about":[3],"/food":[4],"/playlist":[5],"/queries":[6]},K={handleError:({error:s})=>{console.error(s)},reroute:()=>{}}})();export{qt as __tla,J as dictionary,K as hooks,N as matchers,Q as nodes,X as root,Y as server_loads};
