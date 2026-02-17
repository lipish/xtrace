 import { Bell, Globe, PanelLeft, User } from "lucide-react";
 import { Button } from "@/components/ui/button";
 import {
   DropdownMenu,
   DropdownMenuContent,
   DropdownMenuItem,
   DropdownMenuLabel,
   DropdownMenuSeparator,
   DropdownMenuTrigger,
 } from "@/components/ui/dropdown-menu";
 
 interface AppHeaderProps {
   title?: string;
   description?: string;
 }
 
 export function AppHeader({ title, description }: AppHeaderProps) {
   return (
     <header className="flex items-center justify-between h-14 px-6 border-b border-border bg-card">
       <div className="flex items-center gap-4">
         <Button variant="ghost" size="icon" className="h-8 w-8">
           <PanelLeft className="h-4 w-4" />
         </Button>
         {title && (
           <div>
             <h1 className="text-lg font-semibold text-foreground">{title}</h1>
             {description && (
               <p className="text-sm text-muted-foreground">{description}</p>
             )}
           </div>
         )}
       </div>
 
       <div className="flex items-center gap-2">
         <Button variant="ghost" size="icon" className="h-8 w-8">
           <Globe className="h-4 w-4" />
         </Button>
         <Button variant="ghost" size="icon" className="h-8 w-8 relative">
           <Bell className="h-4 w-4" />
           <span className="absolute top-1 right-1 w-2 h-2 bg-xtrace-pink rounded-full" />
         </Button>
         <DropdownMenu>
           <DropdownMenuTrigger asChild>
             <Button variant="ghost" size="icon" className="h-8 w-8">
               <User className="h-4 w-4" />
             </Button>
           </DropdownMenuTrigger>
           <DropdownMenuContent align="end">
             <DropdownMenuLabel>My Account</DropdownMenuLabel>
             <DropdownMenuSeparator />
<DropdownMenuItem>Profile</DropdownMenuItem>
            <DropdownMenuItem>API Keys</DropdownMenuItem>
             <DropdownMenuSeparator />
             <DropdownMenuItem>Sign out</DropdownMenuItem>
           </DropdownMenuContent>
         </DropdownMenu>
       </div>
     </header>
   );
 }