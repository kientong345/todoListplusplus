import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import type { TaskDetailDto, TaskCreateDto } from "@/types/task";

interface TaskCreateFormProps {
  initialData?: TaskDetailDto;
  onSubmit: (data: TaskCreateDto) => Promise<void>;
  onCancel: () => void;
  isLoading?: boolean;
  submitLabel: string;
}

export function TaskCreateForm({ 
  initialData, 
  onSubmit, 
  onCancel, 
  isLoading, 
  submitLabel 
}: TaskCreateFormProps) {
  const [title, setTitle] = useState<string>(initialData?.title || "");
  const [description, setDescription] = useState<string | null>(initialData?.description || null);
  const [cycleTime, setCycleTime] = useState<string | null>(initialData?.cycleTime || null);
  const [expiresAt, setExpiresAt] = useState<string | null>(initialData?.expiresAt || null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    let cycleTimeValue: TaskDetailDto['cycleTime'] = null;
    switch (cycleTime) {
      case "none":
        cycleTimeValue = null;
        break;
      case "daily":
        cycleTimeValue = "0 1 0";
        break;
      case "weekly":
        cycleTimeValue = "0 7 0";
        break;
      case "monthly":
        cycleTimeValue = "1 0 0";
        break;
    }
    onSubmit({
      title,
      description: description || null,
      expiresAt: expiresAt || null,
      cycleTime: cycleTimeValue,
      notifyTime: null,
    });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="title" className="text-sm font-semibold">Task Title</Label>
          <Input
            id="title"
            placeholder="e.g. Finish quarterly report"
            required
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            disabled={isLoading}
            className="h-11 shadow-sm"
          />
        </div>
        
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-2">
            <Label htmlFor="cycleTime" className="text-sm font-semibold">Recurring</Label>
            <Select 
              value={cycleTime || "none"} 
              onValueChange={(value) => setCycleTime(value)}
              disabled={isLoading}
            >
              <SelectTrigger id="cycleTime" className="h-11 shadow-sm">
                <SelectValue placeholder="Select frequency" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="none">None</SelectItem>
                <SelectItem value="daily">Daily</SelectItem>
                <SelectItem value="weekly">Weekly</SelectItem>
                <SelectItem value="monthly">Monthly</SelectItem>
              </SelectContent>
            </Select>
          </div>
          
          <div className="space-y-2">
            <Label htmlFor="expiresAt" className="text-sm font-semibold">Due Date</Label>
            <Input
              id="expiresAt"
              type="date"
              value={expiresAt || ""}
              onChange={(e) => setExpiresAt(e.target.value)}
              disabled={isLoading || (!!cycleTime && cycleTime !== "none")}
              className="h-11 shadow-sm block"
            />
          </div>

        </div>

        <div className="space-y-2">
          <Label htmlFor="description" className="text-sm font-semibold">Description (Optional)</Label>
          <Textarea
            id="description"
            placeholder="Add more details about this task..."
            value={description || ''}
            onChange={(e) => setDescription(e.target.value)}
            disabled={isLoading}
            className="resize-none shadow-sm min-h-[100px]"
          />
        </div>
      </div>
      <div className="flex flex-col-reverse sm:flex-row sm:justify-end gap-3 pt-4">
        <Button 
          type="button" 
          variant="outline" 
          onClick={onCancel} 
          disabled={isLoading}
          className="font-medium"
        >
          Cancel
        </Button>
        <Button 
          type="submit" 
          disabled={isLoading}
          className="font-semibold px-6 shadow-sm"
        >
          {isLoading ? "Saving..." : submitLabel}
        </Button>
      </div>
    </form>
  );
}

