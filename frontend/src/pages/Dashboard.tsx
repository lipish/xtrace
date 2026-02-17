 import { AppLayout } from "@/components/layout/AppLayout";
 import { StatCard } from "@/components/dashboard/StatCard";
 import { Activity, GitBranch, Users, DollarSign, Clock, Sparkles } from "lucide-react";
 import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
 import {
   AreaChart,
   Area,
   XAxis,
   YAxis,
   CartesianGrid,
   Tooltip,
   ResponsiveContainer,
 } from "recharts";
 
 const chartData = [
   { name: "00:00", traces: 120, cost: 0.02 },
   { name: "04:00", traces: 80, cost: 0.015 },
   { name: "08:00", traces: 350, cost: 0.045 },
   { name: "12:00", traces: 520, cost: 0.068 },
   { name: "16:00", traces: 480, cost: 0.062 },
   { name: "20:00", traces: 380, cost: 0.048 },
   { name: "24:00", traces: 220, cost: 0.028 },
 ];
 
 export default function Dashboard() {
   return (
     <AppLayout>
       <div className="space-y-6 animate-fade-in">
         {/* Header */}
         <div>
           <h1 className="text-2xl font-bold text-foreground">Dashboard</h1>
           <p className="text-muted-foreground mt-1">
             Monitor your LLM app performance and usage
           </p>
         </div>
 
         {/* Stats Grid */}
         <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
           <StatCard
             title="Total Traces"
             value="12,847"
             change="+12.5% vs yesterday"
             changeType="positive"
             icon={Activity}
           />
           <StatCard
             title="Active Sessions"
             value="1,234"
             change="+8.2% vs yesterday"
             changeType="positive"
             icon={GitBranch}
           />
           <StatCard
             title="Users"
             value="856"
             change="+3.1% vs yesterday"
             changeType="positive"
             icon={Users}
           />
           <StatCard
             title="Total Cost"
             value="$24.56"
             change="-5.3% vs yesterday"
             changeType="negative"
             icon={DollarSign}
           />
         </div>
 
         {/* Charts Row */}
         <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
           {/* Traces Chart */}
           <Card>
             <CardHeader className="pb-2">
               <CardTitle className="text-base font-medium">Traces Trend</CardTitle>
             </CardHeader>
             <CardContent>
               <div className="h-[280px]">
                 <ResponsiveContainer width="100%" height="100%">
                   <AreaChart data={chartData}>
                     <defs>
                       <linearGradient id="colorTraces" x1="0" y1="0" x2="0" y2="1">
                         <stop offset="5%" stopColor="hsl(266, 92%, 50%)" stopOpacity={0.3} />
                         <stop offset="95%" stopColor="hsl(266, 92%, 50%)" stopOpacity={0} />
                       </linearGradient>
                     </defs>
                     <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                     <XAxis 
                       dataKey="name" 
                       stroke="hsl(var(--muted-foreground))" 
                       fontSize={12}
                       tickLine={false}
                       axisLine={false}
                     />
                     <YAxis 
                       stroke="hsl(var(--muted-foreground))" 
                       fontSize={12}
                       tickLine={false}
                       axisLine={false}
                     />
                     <Tooltip 
                       contentStyle={{
                         backgroundColor: "hsl(var(--card))",
                         border: "1px solid hsl(var(--border))",
                         borderRadius: "8px",
                       }}
                     />
                     <Area
                       type="monotone"
                       dataKey="traces"
                       stroke="hsl(266, 92%, 50%)"
                       strokeWidth={2}
                       fillOpacity={1}
                       fill="url(#colorTraces)"
                     />
                   </AreaChart>
                 </ResponsiveContainer>
               </div>
             </CardContent>
           </Card>
 
           {/* Cost Chart */}
           <Card>
             <CardHeader className="pb-2">
               <CardTitle className="text-base font-medium">Cost Trend</CardTitle>
             </CardHeader>
             <CardContent>
               <div className="h-[280px]">
                 <ResponsiveContainer width="100%" height="100%">
                   <AreaChart data={chartData}>
                     <defs>
                       <linearGradient id="colorCost" x1="0" y1="0" x2="0" y2="1">
                         <stop offset="5%" stopColor="hsl(340, 90%, 55%)" stopOpacity={0.3} />
                         <stop offset="95%" stopColor="hsl(340, 90%, 55%)" stopOpacity={0} />
                       </linearGradient>
                     </defs>
                     <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                     <XAxis 
                       dataKey="name" 
                       stroke="hsl(var(--muted-foreground))" 
                       fontSize={12}
                       tickLine={false}
                       axisLine={false}
                     />
                     <YAxis 
                       stroke="hsl(var(--muted-foreground))" 
                       fontSize={12}
                       tickLine={false}
                       axisLine={false}
                       tickFormatter={(value) => `$${value}`}
                     />
                     <Tooltip 
                       contentStyle={{
                         backgroundColor: "hsl(var(--card))",
                         border: "1px solid hsl(var(--border))",
                         borderRadius: "8px",
                       }}
                       formatter={(value: number) => [`$${value.toFixed(3)}`, "Cost"]}
                     />
                     <Area
                       type="monotone"
                       dataKey="cost"
                       stroke="hsl(340, 90%, 55%)"
                       strokeWidth={2}
                       fillOpacity={1}
                       fill="url(#colorCost)"
                     />
                   </AreaChart>
                 </ResponsiveContainer>
               </div>
             </CardContent>
           </Card>
         </div>
 
         {/* Quick Stats */}
         <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
           <Card>
             <CardContent className="pt-6">
               <div className="flex items-center gap-4">
                 <div className="p-3 rounded-lg bg-xtrace-purple/10">
                   <Sparkles className="h-6 w-6 text-xtrace-purple" />
                 </div>
                 <div>
                   <p className="text-2xl font-bold">2.4M</p>
                   <p className="text-sm text-muted-foreground">Total Tokens Used</p>
                 </div>
               </div>
             </CardContent>
           </Card>
 
           <Card>
             <CardContent className="pt-6">
               <div className="flex items-center gap-4">
                 <div className="p-3 rounded-lg bg-xtrace-orange/10">
                   <Clock className="h-6 w-6 text-xtrace-orange" />
                 </div>
                 <div>
                   <p className="text-2xl font-bold">3.2s</p>
                   <p className="text-sm text-muted-foreground">Avg Latency</p>
                 </div>
               </div>
             </CardContent>
           </Card>
 
           <Card>
             <CardContent className="pt-6">
               <div className="flex items-center gap-4">
                 <div className="p-3 rounded-lg bg-xtrace-success/10">
                   <Activity className="h-6 w-6 text-xtrace-success" />
                 </div>
                 <div>
                   <p className="text-2xl font-bold">99.2%</p>
                   <p className="text-sm text-muted-foreground">Success Rate</p>
                 </div>
               </div>
             </CardContent>
           </Card>
         </div>
       </div>
     </AppLayout>
   );
 }