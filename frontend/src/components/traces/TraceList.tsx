import { useMemo, useState } from "react";
import { cn } from "@/lib/utils";
import { Search, Filter, ChevronRight, Clock, DollarSign, ArrowRightLeft } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import type { TraceListItem } from "@/lib/xtrace";

export type TraceListItemView = TraceListItem & {
  latencyText: string;
  costText: string;
  tokenText: string;
  status: "success" | "error" | "pending";
};

interface TraceListProps {
  traces: TraceListItemView[];
  onSelectTrace: (trace: TraceListItemView) => void;
  selectedTraceId?: string;
}

export function TraceList({ traces, onSelectTrace, selectedTraceId }: TraceListProps) {
  const [searchQuery, setSearchQuery] = useState("");

  const filteredTraces = useMemo(() => {
    const query = searchQuery.trim().toLowerCase();
    if (!query) return traces;
    return traces.filter(
      (trace) =>
        (trace.name || "").toLowerCase().includes(query) ||
        trace.id.toLowerCase().includes(query)
    );
  }, [searchQuery, traces]);

  return (
    <div className="flex flex-col h-full bg-card rounded-lg border border-border">
      <div className="p-4 border-b border-border">
        <div className="flex items-center gap-2">
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              placeholder="Search (name, ID)"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-9"
            />
          </div>
          <Button variant="outline" size="icon" disabled>
            <Filter className="h-4 w-4" />
          </Button>
        </div>
        <div className="flex items-center justify-between mt-3">
          <span className="text-sm text-muted-foreground">{filteredTraces.length} total</span>
          <div className="flex items-center gap-2">
            <Button variant="ghost" size="sm" className="h-7 text-xs" disabled>
              Timeline
            </Button>
          </div>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto scrollbar-thin">
        {filteredTraces.map((trace) => (
          <div
            key={trace.id}
            onClick={() => onSelectTrace(trace)}
            className={cn(
              "px-4 py-3 border-b border-border cursor-pointer transition-colors hover:bg-accent/50",
              selectedTraceId === trace.id && "bg-accent"
            )}
          >
            <div className="flex items-start justify-between">
              <div className="flex items-center gap-2">
                <div
                  className={cn(
                    "w-1.5 h-1.5 rounded-full",
                    trace.status === "success" && "bg-xtrace-success",
                    trace.status === "error" && "bg-destructive",
                    trace.status === "pending" && "bg-xtrace-warning animate-pulse-subtle"
                  )}
                />
                <span className="font-medium text-foreground">{trace.name || "(unnamed)"}</span>
              </div>
              <ChevronRight className="h-4 w-4 text-muted-foreground" />
            </div>

            <div className="mt-2 flex items-center gap-3 text-xs text-muted-foreground">
              <span className="flex items-center gap-1">
                <Clock className="h-3 w-3" />
                {trace.latencyText}
              </span>
              <span className="flex items-center gap-1">
                <DollarSign className="h-3 w-3" />
                {trace.costText}
              </span>
              <span className="flex items-center gap-1">
                <ArrowRightLeft className="h-3 w-3" />
                {trace.tokenText}
              </span>
            </div>

            <div className="mt-2 flex items-center gap-2">
              {trace.sessionId && (
                <Badge variant="secondary" className="text-[10px] h-5">
                  {trace.sessionId}
                </Badge>
              )}
              {trace.userId && (
                <Badge variant="outline" className="text-[10px] h-5">
                  {trace.userId}
                </Badge>
              )}
            </div>

            <div className="mt-1.5 text-[11px] text-muted-foreground">
              {trace.timestamp || "-"}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
