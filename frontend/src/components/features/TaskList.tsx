import { useState } from "react";
import { Badge } from "@/components/ui/badge";
import { Checkbox } from "@/components/ui/checkbox";
import { Card, CardContent } from "@/components/ui/card";
import { Clock, MoreVertical, Edit2, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { Task } from "@/types";
import { 
  DropdownMenu, 
  DropdownMenuContent, 
  DropdownMenuItem, 
  DropdownMenuTrigger 
} from "@/components/ui/dropdown-menu";
import { 
  Dialog, 
  DialogContent, 
  DialogDescription, 
  DialogHeader, 
  DialogTitle 
} from "@/components/ui/dialog";
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
import { TaskForm } from "./TaskForm";

interface TaskListProps {
  tasks: Task[];
  onToggleStatus: (taskId: string) => void;
  onUpdate?: (taskId: string, data: Partial<Task>) => void;
  onDelete?: (taskId: string) => void;
}

export function TaskList({ tasks, onToggleStatus, onUpdate, onDelete }: TaskListProps) {
  const [editingTask, setEditingTask] = useState<Task | null>(null);
  const [deletingTask, setDeletingTask] = useState<Task | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const handleUpdate = (data: Partial<Task>) => {
    if (!editingTask) return;
    setIsLoading(true);
    // TODO: API interaction
    setTimeout(() => {
      onUpdate?.(editingTask.id, data);
      setIsLoading(false);
      setEditingTask(null);
    }, 1000);
  };

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
                   <Badge variant="outline" className="text-xs capitalize">
                    {task.cycleTime}
                   </Badge>
                ) : task.expiresAt ? (
                  <div className="flex items-center text-xs text-muted-foreground bg-muted px-2 py-1 rounded-md whitespace-nowrap">
                    <Clock className="w-3 h-3 mr-1" />
                    {formatDate(task.expiresAt)}
                  </div>
                ) : null}
              </div>

              <div className="flex justify-end">
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <Button variant="ghost" size="icon" className="h-8 w-8 text-muted-foreground hover:text-foreground">
                      <MoreVertical className="h-4 w-4" />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end" className="rounded-xl">
                    <DropdownMenuItem onClick={() => setEditingTask(task)}>
                      <Edit2 className="mr-2 h-4 w-4" /> Edit
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => setDeletingTask(task)} className="text-destructive focus:text-destructive">
                      <Trash2 className="mr-2 h-4 w-4" /> Delete
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>
            </CardContent>
          </Card>
        ))

      )}

      {/* Edit Task Dialog */}
      <Dialog open={!!editingTask} onOpenChange={(open) => !open && setEditingTask(null)}>
        <DialogContent className="sm:max-w-[500px] rounded-2xl">
          <DialogHeader>
            <DialogTitle className="text-2xl font-bold">Edit Task</DialogTitle>
            <DialogDescription>Make changes to your task.</DialogDescription>
          </DialogHeader>
          <TaskForm
            initialData={editingTask || {}}
            onSubmit={handleUpdate}
            onCancel={() => setEditingTask(null)}
            submitLabel="Save Changes"
            isLoading={isLoading}
          />
        </DialogContent>
      </Dialog>

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

