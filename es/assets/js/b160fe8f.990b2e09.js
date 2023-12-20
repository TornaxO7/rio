"use strict";(self.webpackChunkrio_docs=self.webpackChunkrio_docs||[]).push([[9152],{255:(o,e,a)=>{a.r(e),a.d(e,{assets:()=>l,contentTitle:()=>r,default:()=>p,frontMatter:()=>i,metadata:()=>s,toc:()=>c});var n=a(5893),t=a(1151);const i={title:"Color automation for navigation",language:"en"},r=void 0,s={id:"features/color-automation-for-navigation",title:"Color automation for navigation",description:"Rio allows to specify color overwrites for tabs based on program and path contexts, using the program and path options.",source:"@site/docs/features/color-automation-for-navigation.md",sourceDirName:"features",slug:"/features/color-automation-for-navigation",permalink:"/rio/es/docs/features/color-automation-for-navigation",draft:!1,unlisted:!1,editUrl:"https://github.com/raphamorim/rio/tree/main/docs/docs/features/color-automation-for-navigation.md",tags:[],version:"current",frontMatter:{title:"Color automation for navigation",language:"en"},sidebar:"tutorialSidebar",previous:{title:"Adaptive theme",permalink:"/rio/es/docs/features/adaptive-theme"},next:{title:"Hyperlinks",permalink:"/rio/es/docs/features/hyperlinks"}},l={},c=[{value:"Program",id:"program",level:4},{value:"Path",id:"path",level:4},{value:"Program and path",id:"program-and-path",level:4}];function d(o){const e={code:"code",h4:"h4",p:"p",pre:"pre",...(0,t.a)(),...o.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsxs)(e.p,{children:["Rio allows to specify color overwrites for tabs based on program and path contexts, using the ",(0,n.jsx)(e.code,{children:"program"})," and ",(0,n.jsx)(e.code,{children:"path"})," options."]}),"\n",(0,n.jsxs)(e.p,{children:["It is possible to use ",(0,n.jsx)(e.code,{children:"program"})," and ",(0,n.jsx)(e.code,{children:"path"})," at the same time."]}),"\n",(0,n.jsxs)(e.p,{children:["Note: ",(0,n.jsx)(e.code,{children:"path"})," is only available for MacOS, BSD and Linux."]}),"\n",(0,n.jsx)(e.h4,{id:"program",children:"Program"}),"\n",(0,n.jsxs)(e.p,{children:["The example below sets ",(0,n.jsx)(e.code,{children:"#FFFF00"})," as color background whenever ",(0,n.jsx)(e.code,{children:"nvim"})," is running."]}),"\n",(0,n.jsxs)("p",{children:[(0,n.jsx)("img",{alt:"example navigation with program color automation using TopTab",src:"/rio/assets/features/demo-colorized-navigation.png",width:"48%"}),(0,n.jsx)("img",{alt:"example navigation with program color automation using CollapsedTab",src:"/rio/assets/features/demo-colorized-navigation-2.png",width:"48%"})]}),"\n",(0,n.jsx)(e.p,{children:"The configuration would be like:"}),"\n",(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{className:"language-toml",children:'[navigation]\ncolor-automation = [\n  { program = "nvim", color = "#FFFF00" }\n]\n'})}),"\n",(0,n.jsx)(e.h4,{id:"path",children:"Path"}),"\n",(0,n.jsxs)(e.p,{children:["The example below sets ",(0,n.jsx)(e.code,{children:"#FFFF00"})," as color background when in the ",(0,n.jsx)(e.code,{children:"/home/geg/.config/rio"})," path."]}),"\n",(0,n.jsxs)(e.p,{children:["Note: ",(0,n.jsx)(e.code,{children:"path"})," is only available for MacOS, BSD and Linux."]}),"\n",(0,n.jsx)(e.p,{children:"The configuration would be like:"}),"\n",(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{className:"language-toml",children:'[navigation]\ncolor-automation = [\n  { path = "/home/geg/.config/rio", color = "#FFFF00" }\n]\n'})}),"\n",(0,n.jsxs)("p",{children:[(0,n.jsx)("img",{alt:"example navigation with path color automation using TopTab",src:"/rio/assets/features/demo-colorized-navigation-path-1.png",width:"48%"}),(0,n.jsx)("img",{alt:"example navigation with path color automation using CollapsedTab",src:"/rio/assets/features/demo-colorized-navigation-path-2.png",width:"48%"})]}),"\n",(0,n.jsx)(e.h4,{id:"program-and-path",children:"Program and path"}),"\n",(0,n.jsxs)(e.p,{children:["It is also possible to use both ",(0,n.jsx)(e.code,{children:"path"})," and ",(0,n.jsx)(e.code,{children:"program"})," at the same time."]}),"\n",(0,n.jsxs)(e.p,{children:["The example below sets ",(0,n.jsx)(e.code,{children:"#FFFF00"})," as color background when in the ",(0,n.jsx)(e.code,{children:"/home"})," path and ",(0,n.jsx)(e.code,{children:"nvim"})," is open."]}),"\n",(0,n.jsxs)(e.p,{children:["Note: ",(0,n.jsx)(e.code,{children:"path"})," is only available for MacOS, BSD and Linux."]}),"\n",(0,n.jsx)(e.p,{children:"The configuration would be like:"}),"\n",(0,n.jsx)(e.pre,{children:(0,n.jsx)(e.code,{className:"language-toml",children:'[navigation]\ncolor-automation = [\n  { program = "nvim", path = "/home", color = "#FFFF00" }\n]\n'})}),"\n",(0,n.jsxs)("p",{children:[(0,n.jsx)("img",{alt:"example navigation with program and path color automation using TopTab",src:"/rio/assets/features/demo-colorized-navigation-program-and-path-1.png",width:"48%"}),(0,n.jsx)("img",{alt:"example navigation with program and path color automation using CollapsedTab",src:"/rio/assets/features/demo-colorized-navigation-program-and-path-2.png",width:"48%"})]})]})}function p(o={}){const{wrapper:e}={...(0,t.a)(),...o.components};return e?(0,n.jsx)(e,{...o,children:(0,n.jsx)(d,{...o})}):d(o)}},1151:(o,e,a)=>{a.d(e,{Z:()=>s,a:()=>r});var n=a(7294);const t={},i=n.createContext(t);function r(o){const e=n.useContext(i);return n.useMemo((function(){return"function"==typeof o?o(e):{...e,...o}}),[e,o])}function s(o){let e;return e=o.disableParentContext?"function"==typeof o.components?o.components(t):o.components||t:r(o.components),n.createElement(i.Provider,{value:e},o.children)}}}]);