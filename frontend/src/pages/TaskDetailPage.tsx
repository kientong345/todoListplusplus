import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { tasks as taskService } from "@/services/api";
import { TaskDetail } from "@/components/features/TaskDetail";
import { Loader2, AlertCircle } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { TaskDetailDto, TaskUpdateDto } from "@/types/task";

export default function TaskDetailPage() {
  const { categoryId, taskId } = useParams<{ categoryId: string; taskId: string }>();
  const navigate = useNavigate();

  const [task, setTask] = useState<TaskDetailDto | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchTask = async () => {
    if (!categoryId || !taskId) return;
    setIsLoading(true);
    setError(null);
    try {
      const data = await taskService.getOne(categoryId, taskId);
      setTask(data);
    } catch (err) {
      console.error("Failed to fetch task:", err);
      setError("Could not load task details. It might have been deleted.");
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchTask();
  }, [categoryId, taskId]);

  const handleUpdateField = async (field: keyof TaskUpdateDto, value: any) => {
     if (!categoryId || !taskId || !task) return;
     try {
       // Using optimistic update for snappier feel
       setTask({ ...task, [field]: value });
       await taskService.update(categoryId, taskId, { [field]: value });
       // No need to fetchTask() here if we trust the optimistic update, 
       // but we could if we want to sync other possible changes (like updatedAt)
       // fetchTask(); 
     } catch (err) {
       console.error(`Failed to update task ${field}:`, err);
       // Revert optimistic update on error
       fetchTask();
     }
  };

  if (isLoading && !task) {
    return (
      <div className="flex h-[60vh] items-center justify-center">
        <Loader2 className="h-10 w-10 animate-spin text-primary" />
      </div>
    );
  }

  if (error || !task) {
    return (
      <div className="flex flex-col items-center justify-center h-[60vh] space-y-4">
        <div className="bg-destructive/10 p-4 rounded-full">
          <AlertCircle className="w-10 h-10 text-destructive" />
        </div>
        <h2 className="text-xl font-bold">{error || "Task not found"}</h2>
        <Button onClick={() => navigate(`/categories/${categoryId}`)}>
          Back to Category
        </Button>
      </div>
    );
  }

  return (
    <div className="py-8">
      <TaskDetail 
        task={task} 
        onUpdateField={handleUpdateField}
        onBack={() => navigate(`/categories/${categoryId}`)}
      />
    </div>
  );
}
