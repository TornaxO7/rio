"use strict";(self.webpackChunkrio_docs=self.webpackChunkrio_docs||[]).push([[6687],{6107:(e,s,r)=>{r.r(s),r.d(s,{assets:()=>d,contentTitle:()=>o,default:()=>u,frontMatter:()=>a,metadata:()=>l,toc:()=>h});var t=r(5893),i=r(1151),n=r(1307);const a={layout:"post",title:"Release 0.0.6",date:"2023-06-07 10:34:14 +0200",categories:"macos linux release",description:"Tabs support, underline and beam cursor, text styles and many bug fixes."},o="Tabs support, underline and beam cursor, text styles and many bug fixes.",l={permalink:"/rio/es/blog/2023/06/07/release-0.0.6",editUrl:"https://github.com/raphamorim/rio/tree/main/docs/blog/2023-06-07-release-0.0.6.mdx",source:"@site/blog/2023-06-07-release-0.0.6.mdx",title:"Release 0.0.6",description:"Tabs support, underline and beam cursor, text styles and many bug fixes.",date:"2023-06-07T10:34:14.000Z",formattedDate:"7 de junio de 2023",tags:[],readingTime:1.355,hasTruncateMarker:!1,authors:[],frontMatter:{layout:"post",title:"Release 0.0.6",date:"2023-06-07 10:34:14 +0200",categories:"macos linux release",description:"Tabs support, underline and beam cursor, text styles and many bug fixes."},unlisted:!1,prevItem:{title:"Rio 0.0.8",permalink:"/rio/es/blog/2023/07/10/release-0.0.8"},nextItem:{title:"Release 0.0.5",permalink:"/rio/es/blog/2023/05/31/release-0.0.5"}},d={authorsImageUrls:[]},h=[{value:"Underline and strikethrough style",id:"underline-and-strikethrough-style",level:2},{value:"Tabs support",id:"tabs-support",level:2},{value:"Support to Beam and Underline cursors",id:"support-to-beam-and-underline-cursors",level:2},{value:"Changelog",id:"changelog",level:2}];function c(e){const s={a:"a",h2:"h2",li:"li",p:"p",ul:"ul",...(0,i.a)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(s.p,{children:"Rio release 0.0.6 is finally here, there's a lot of updates to cover so let's get started. I also would like to remind you that Rio is still unstable. This release adds a lot of bug fixes and feature additions in order to make Rio terminal stable."}),"\n",(0,t.jsx)(s.h2,{id:"underline-and-strikethrough-style",children:"Underline and strikethrough style"}),"\n",(0,t.jsx)(s.p,{children:'Support to text styling as such "Underline" and "Strikethrough".'}),"\n",(0,t.jsx)(s.h2,{id:"tabs-support",children:"Tabs support"}),"\n",(0,t.jsx)(s.p,{children:"Tabs has been added to Rio terminal for macos and linux platform."}),"\n",(0,t.jsx)(s.p,{children:"The shortcuts:"}),"\n",(0,t.jsxs)(s.ul,{children:["\n",(0,t.jsx)(s.li,{children:"Create tab: Logo key (Command in macos) + T."}),"\n",(0,t.jsx)(s.li,{children:"Close tab: Logo key (Command in macos) + W."}),"\n",(0,t.jsx)(s.li,{children:"Switch tab: Control key + Tab key."}),"\n"]}),"\n",(0,t.jsx)(s.p,{children:"Below you can see an example of usage:"}),"\n",(0,t.jsx)(n.t,{id:"1664585160958922755"}),"\n",(0,t.jsx)(s.p,{children:"Note: There's a limit of maximum of 6 tabs for now."}),"\n",(0,t.jsx)(s.h2,{id:"support-to-beam-and-underline-cursors",children:"Support to Beam and Underline cursors"}),"\n",(0,t.jsx)(s.p,{children:"Beam and underline cursor support has been added to Rio terminal. Also, block cursor and IME state allow a character to be visible."}),"\n",(0,t.jsx)(n.t,{id:"1664146499398139906"}),"\n",(0,t.jsx)(s.h2,{id:"changelog",children:"Changelog"}),"\n",(0,t.jsxs)(s.ul,{children:["\n",(0,t.jsxs)(s.li,{children:["Fix: support to clipboard in linux by ",(0,t.jsx)(s.a,{href:"https://github.com/joseemds",children:"@joseemds"}),"."]}),"\n",(0,t.jsxs)(s.li,{children:["Font style for custom fonts by ",(0,t.jsx)(s.a,{href:"https://github.com/OlshaMB",children:"@OlshaMB"})," (closed ",(0,t.jsx)(s.a,{href:"https://github.com/raphamorim/rio/issues/80",children:"#80"})," and ",(0,t.jsx)(s.a,{href:"https://github.com/raphamorim/rio/issues/81",children:"#81"}),")"]}),"\n",(0,t.jsxs)(s.li,{children:["Text styles Underline and Strikethrough (closed ",(0,t.jsx)(s.a,{href:"https://github.com/raphamorim/rio/issues/79",children:"#79"}),")."]}),"\n",(0,t.jsx)(s.li,{children:"Update default colors for tabs/tabs-active."}),"\n",(0,t.jsx)(s.li,{children:"Tabs support."}),"\n",(0,t.jsxs)(s.li,{children:["Fix rendering tab and hidden chars by replacing to space by ",(0,t.jsx)(s.a,{href:"https://github.com/niuez",children:"@niuez"}),", (closed ",(0,t.jsx)(s.a,{href:"https://github.com/raphamorim/rio/issues/56",children:"#56"}),")."]}),"\n",(0,t.jsx)(s.li,{children:"Block cursor hover a character and still allow it to be visible."}),"\n",(0,t.jsxs)(s.li,{children:["Support to caret Beam and Underline cursor ",(0,t.jsx)(s.a,{href:"https://github.com/raphamorim/rio/issues/67",children:"#67"})," by ",(0,t.jsx)(s.a,{href:"https://github.com/niuez",children:"@niuez"}),"."]}),"\n",(0,t.jsxs)(s.li,{children:["Fix panics if custom font is not found ",(0,t.jsx)(s.a,{href:"https://github.com/raphamorim/rio/issues/68",children:"#68"}),"."]}),"\n",(0,t.jsx)(s.li,{children:"MacOs ignore alt key in cntrlseq (same behavior as Terminal.app, Hyper, iTerm and etecetera)."}),"\n"]})]})}function u(e={}){const{wrapper:s}={...(0,i.a)(),...e.components};return s?(0,t.jsx)(s,{...e,children:(0,t.jsx)(c,{...e})}):c(e)}}}]);