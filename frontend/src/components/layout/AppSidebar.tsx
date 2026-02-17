 import { useState } from "react";
 import { Link, useLocation } from "react-router-dom";
 import {
   LayoutDashboard,
   Activity,
   GitBranch,
   Users,
   FileText,
   Play,
   Star,
   FlaskConical,
   MessageSquare,
   Database,
   Settings,
   HelpCircle,
   ChevronDown,
   Search,
   Command,
 } from "lucide-react";
 import { cn } from "@/lib/utils";
 import { Input } from "@/components/ui/input";
 
 interface NavItem {
   label: string;
   icon: React.ElementType;
   href: string;
   badge?: string;
 }
 
 interface NavGroup {
   title: string;
   items: NavItem[];
 }
 
 const navGroups: NavGroup[] = [
   {
     title: "",
     items: [
       { label: "Dashboard", icon: LayoutDashboard, href: "/" },
     ],
   },
   {
     title: "Observability",
     items: [
       { label: "Traces", icon: Activity, href: "/traces" },
       { label: "Sessions", icon: GitBranch, href: "/sessions" },
       { label: "Users", icon: Users, href: "/users" },
     ],
   },
   {
     title: "Prompt Management",
     items: [
       { label: "Prompts", icon: FileText, href: "/prompts" },
       { label: "Playground", icon: Play, href: "/playground" },
     ],
   },
   {
     title: "Evaluation",
     items: [
       { label: "Scores", icon: Star, href: "/scores" },
{ label: "LLM Evaluation", icon: FlaskConical, href: "/llm-judge" },
      { label: "Annotations", icon: MessageSquare, href: "/annotations" },
      { label: "Datasets", icon: Database, href: "/datasets" },
     ],
   },
 ];
 
 const bottomItems: NavItem[] = [
{ label: "Settings", icon: Settings, href: "/settings" },
  { label: "Help", icon: HelpCircle, href: "/help" },
 ];
 
 export function AppSidebar() {
   const location = useLocation();
   const [collapsed, setCollapsed] = useState(false);
 
   const isActive = (href: string) => {
     if (href === "/") return location.pathname === "/";
     return location.pathname.startsWith(href);
   };
 
   return (
     <aside
       className={cn(
         "flex flex-col h-screen bg-sidebar border-r border-sidebar-border transition-all duration-300",
         collapsed ? "w-16" : "w-64"
       )}
     >
       {/* Logo */}
       <div className="flex items-center gap-3 px-4 h-14 border-b border-sidebar-border">
         <div className="flex items-center justify-center w-8 h-8">
           <img src="/favicon.svg" alt="XTrace" className="w-8 h-8" />
         </div>
         {!collapsed && (
           <div className="flex items-center gap-2">
             <span className="font-semibold text-foreground text-lg">XTrace</span>
             <span className="text-xs text-muted-foreground bg-muted px-1.5 py-0.5 rounded">v1.0.0</span>
           </div>
         )}
       </div>
 
       {/* Search */}
       {!collapsed && (
         <div className="px-3 py-3">
           <div className="relative">
             <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
             <Input
               placeholder="Search..."
               className="pl-8 pr-8 h-8 bg-secondary border-0 text-sm"
             />
             <div className="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-0.5">
               <kbd className="text-[10px] text-muted-foreground bg-muted px-1 rounded">⌘</kbd>
               <kbd className="text-[10px] text-muted-foreground bg-muted px-1 rounded">K</kbd>
             </div>
           </div>
         </div>
       )}
 
       {/* Navigation */}
       <nav className="flex-1 overflow-y-auto py-2 scrollbar-thin">
         {navGroups.map((group, groupIndex) => (
           <div key={groupIndex} className="mb-2">
             {group.title && !collapsed && (
               <div className="px-4 py-2 text-xs font-medium text-muted-foreground uppercase tracking-wider">
                 {group.title}
               </div>
             )}
             {group.items.map((item) => {
               const Icon = item.icon;
               const active = isActive(item.href);
               return (
                 <Link
                   key={item.href}
                   to={item.href}
                   className={cn(
                     "flex items-center gap-3 mx-2 px-2 py-2 rounded-md text-sm font-medium transition-colors",
                     active
                       ? "bg-sidebar-accent text-sidebar-accent-foreground"
                       : "text-sidebar-foreground hover:bg-sidebar-accent/50 hover:text-sidebar-accent-foreground"
                   )}
                 >
                   <Icon className={cn("h-4 w-4 shrink-0", active && "text-primary")} />
                   {!collapsed && <span>{item.label}</span>}
                   {!collapsed && item.badge && (
                     <span className="ml-auto text-xs bg-primary text-primary-foreground px-1.5 py-0.5 rounded">
                       {item.badge}
                     </span>
                   )}
                 </Link>
               );
             })}
           </div>
         ))}
       </nav>
 
       {/* Bottom items */}
       <div className="border-t border-sidebar-border py-2">
         {bottomItems.map((item) => {
           const Icon = item.icon;
           const active = isActive(item.href);
           return (
             <Link
               key={item.href}
               to={item.href}
               className={cn(
                 "flex items-center gap-3 mx-2 px-2 py-2 rounded-md text-sm font-medium transition-colors",
                 active
                   ? "bg-sidebar-accent text-sidebar-accent-foreground"
                   : "text-sidebar-foreground hover:bg-sidebar-accent/50"
               )}
             >
               <Icon className="h-4 w-4 shrink-0" />
               {!collapsed && <span>{item.label}</span>}
             </Link>
           );
         })}
       </div>
 
       {/* User */}
       <div className="border-t border-sidebar-border p-3">
         <div className={cn("flex items-center gap-3", collapsed && "justify-center")}>
           <div className="w-8 h-8 rounded-full gradient-brand flex items-center justify-center text-white text-sm font-medium">
             U
           </div>
           {!collapsed && (
             <div className="flex-1 min-w-0">
               <div className="text-sm font-medium text-foreground truncate">Username</div>
               <div className="text-xs text-muted-foreground truncate">user@example.com</div>
             </div>
           )}
           {!collapsed && (
             <ChevronDown className="h-4 w-4 text-muted-foreground shrink-0" />
           )}
         </div>
       </div>
     </aside>
   );
 }