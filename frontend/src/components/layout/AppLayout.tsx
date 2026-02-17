 import { AppSidebar } from "./AppSidebar";
 import { AppHeader } from "./AppHeader";
 
 interface AppLayoutProps {
   children: React.ReactNode;
   title?: string;
   description?: string;
 }
 
 export function AppLayout({ children, title, description }: AppLayoutProps) {
   return (
     <div className="flex h-screen bg-background overflow-hidden">
       <AppSidebar />
       <div className="flex-1 flex flex-col min-w-0 overflow-hidden">
         <AppHeader title={title} description={description} />
         <main className="flex-1 overflow-auto p-6">
           {children}
         </main>
       </div>
     </div>
   );
 }