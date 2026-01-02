import { useState } from "react";
import { Badge } from "@/components/ui/badge";
import { Link } from "react-router-dom";
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
  categoryId: string;
  tasks: TaskMinimalDto[];
  onDelete?: (taskId: string) => void;
}

export function TaskList({ categoryId, tasks, onDelete }: TaskListProps) {
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
      case 'done': return 'bg-green-500 hover:bg-green-600';
      case 'cancel': return 'bg-red-500 hover:bg-red-600';
      default: return 'bg-gray-500';
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case 'open': return 'Open';
      case 'done': return 'Done';
      case 'cancel': return 'Cancel';
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
          <Card key={task.id} className="group hover:shadow-md transition-all border-muted relative">
            <CardContent className="p-4 grid grid-cols-[1fr_auto_auto_40px] gap-6 items-center">
              <div className="min-w-0">
                <Link to={`/categories/${categoryId}/tasks/${task.id}`}>
                  <h4 className={`font-semibold text-lg truncate hover:text-primary transition-colors ${task.status === 'done' ? 'text-muted-foreground line-through' : 'text-foreground'}`}>
                    {task.title}
                  </h4>
                </Link>
              </div>

              <div className="flex justify-end">
                {task.cycleTime && task.cycleTime !== 'none' ? (
                   <Badge variant="outline" className={`${getReccurrenceColor(task.cycleTime)} border-none text-white text-xs capitalize font-medium py-1 px-2.5 rounded-full`}>
                    {getReccurrenceLabel(task.cycleTime)}
                   </Badge>
                ) : task.expiresAt ? (
                  <div className="flex items-center text-xs font-semibold text-muted-foreground bg-muted/50 px-3 py-1.5 rounded-full whitespace-nowrap">
                    <Clock className="w-3.5 h-3.5 mr-1.5" />
                    {formatDate(task.expiresAt)}
                  </div>
                ) : null}
              </div>

              <div className="flex justify-center">
                <Badge className={`${getStatusColor(task.status)} border-none shadow-none min-w-[80px] justify-center font-medium`}>
                  {getStatusLabel(task.status)}
                </Badge>
              </div>

              <div className="flex justify-end opacity-0 group-hover:opacity-100 transition-opacity">
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-9 w-9 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-full transition-all"
                  onClick={() => setDeletingTask(task)}
                >
                  <Trash2 className="h-4 w-4" />
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

