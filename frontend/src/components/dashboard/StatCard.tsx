 import { cn } from "@/lib/utils";
 import { LucideIcon } from "lucide-react";
 
 interface StatCardProps {
   title: string;
   value: string | number;
   change?: string;
   changeType?: "positive" | "negative" | "neutral";
   icon: LucideIcon;
   className?: string;
 }
 
 export function StatCard({
   title,
   value,
   change,
   changeType = "neutral",
   icon: Icon,
   className,
 }: StatCardProps) {
   return (
     <div
       className={cn(
         "bg-card rounded-lg border border-border p-5 transition-shadow hover:shadow-md",
         className
       )}
     >
       <div className="flex items-start justify-between">
         <div>
           <p className="text-sm font-medium text-muted-foreground">{title}</p>
           <p className="text-2xl font-bold text-foreground mt-1">{value}</p>
           {change && (
             <p
               className={cn(
                 "text-xs mt-2",
                 changeType === "positive" && "text-xtrace-success",
                 changeType === "negative" && "text-destructive",
                 changeType === "neutral" && "text-muted-foreground"
               )}
             >
               {change}
             </p>
           )}
         </div>
         <div className="p-2.5 rounded-lg bg-accent">
           <Icon className="h-5 w-5 text-primary" />
         </div>
       </div>
     </div>
   );
 }