import { useState } from "react";
import { Badge } from "@/components/ui/badge";
import { Checkbox } from "@/components/ui/checkbox";
import { Card, CardContent } from "@/components/ui/card";
import { Clock, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { TaskMinimalDto } from "@/types/task";

import { 
  AlertDialog, 
  AlertDialogAction, 
  AlertDialogCancel, 
  AlertDialogContent, 
  AlertDialogDescription, 
  AlertDialogFooter, 
  AlertDialogHeader, 
  AlertDialogTitle 
} from "@/components/ui/alert-dialog";

interface TaskListProps {
  tasks: TaskMinimalDto[];
  onToggleStatus: (taskId: string) => void;
  onDelete?: (taskId: string) => void;
}

export function TaskList({ tasks, onToggleStatus, onDelete }: TaskListProps) {
  const [deletingTask, setDeletingTask] = useState<TaskMinimalDto | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const handleDelete = () => {
    if (!deletingTask) return;
    setIsLoading(true);
    // TODO: API interaction
    setTimeout(() => {
      onDelete?.(deletingTask.id);
      setIsLoading(false);
      setDeletingTask(null);
    }, 1000);
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'open': return 'bg-blue-500 hover:bg-blue-600';
      case 'in_progress': return 'bg-amber-500 hover:bg-amber-600';
      case 'done': return 'bg-green-500 hover:bg-green-600';
      case 'canceled': return 'bg-red-500 hover:bg-red-600';
      default: return 'bg-gray-500';
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case 'open': return 'Open';
      case 'in_progress': return 'In Progress';
      case 'done': return 'Done';
      case 'canceled': return 'Canceled';
      default: return status;
    }
  };

  const getReccurrenceColor = (cycleTime: string | null) => {
    switch (cycleTime) {
      case '0 1 0': return 'bg-amber-500 hover:bg-amber-600';
      case '0 7 0': return 'bg-green-500 hover:bg-green-600';
      case '1 0 0': return 'bg-pink-500 hover:bg-pink-600';
      default: return 'bg-gray-500';
    }
  };

  const getReccurrenceLabel = (cycleTime: string | null) => {
    switch (cycleTime) {
      case '0 1 0': return 'Daily';
      case '0 7 0': return 'Weekly';
      case '1 0 0': return 'Monthly';
      default: return 'None';
    }
  };


  const formatDate = (dateString?: string | null) => {
    if (!dateString) return null;
    const date = new Date(dateString);
    return date.toLocaleDateString("en-US", { month: "short", day: "numeric" });
  };

  return (
    <div className="space-y-3">
      {tasks.length === 0 ? (
        <div className="text-center py-12 border-2 border-dashed rounded-xl">
          <p className="text-muted-foreground">No tasks in this category yet.</p>
        </div>
      ) : (
        tasks.map((task) => (
          <Card key={task.id} className="group hover:shadow-sm transition-all border-muted">
            <CardContent className="p-4 grid grid-cols-[auto_1fr_120px_120px_auto] gap-4 items-center">
              <Checkbox 
                checked={task.status === 'done'} 
                onCheckedChange={() => onToggleStatus(task.id)}
                className="w-5 h-5 rounded-md border-2" 
              />
              
              <div className="min-w-0">
                <h4 className={`font-medium truncate ${task.status === 'done' ? 'text-muted-foreground line-through' : ''}`}>
                  {task.title}
                </h4>
              </div>

              <div className="flex justify-center">
                <Badge className={`${getStatusColor(task.status)} border-none shadow-none w-24 justify-center`}>
                  {getStatusLabel(task.status)}
                </Badge>
              </div>

              <div className="flex justify-end">
                {task.cycleTime && task.cycleTime !== 'none' ? (
                   <Badge variant="outline" className={`${getReccurrenceColor(task.cycleTime)} text-xs capitalize`}>
                    {getReccurrenceLabel(task.cycleTime)}
                   </Badge>
                ) : task.expiresAt ? (
                  <div className="flex items-center text-xs text-muted-foreground bg-muted px-2 py-1 rounded-md whitespace-nowrap">
                    <Clock className="w-3 h-3 mr-1" />
                    {formatDate(task.expiresAt)}
                  </div>
                ) : null}
              </div>

              <div className="flex justify-end">
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-8 w-8 text-muted-foreground hover:text-foreground"
                  onClick={() => setDeletingTask(task)}
                >
                  <Trash2 className="mr-2 h-4 w-4 text-destructive focus:text-destructive" />
                </Button>
              </div>
            </CardContent>
          </Card>
        ))

      )}

      {/* Delete Confirmation */}
      <AlertDialog open={!!deletingTask} onOpenChange={(open) => !open && setDeletingTask(null)}>
        <AlertDialogContent className="rounded-2xl">
          <AlertDialogHeader>
            <AlertDialogTitle>Delete this task?</AlertDialogTitle>
            <AlertDialogDescription>
              "{deletingTask?.title}" will be permanently removed.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={handleDelete} className="bg-destructive text-destructive-foreground hover:bg-destructive/90">
              {isLoading ? "Deleting..." : "Delete"}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}

