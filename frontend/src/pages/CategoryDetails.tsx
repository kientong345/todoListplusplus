import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { api } from "@/lib/api";
import type { CategoryDetail } from "@/types/category";
import type { TaskMinimal, TaskDetail } from "@/types/task";
import type { PaginatedResponse } from "@/types/common";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Plus, ArrowLeft, Pencil, Trash, CheckCircle, XCircle, Circle, MoreVertical } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Badge } from "@/components/ui/badge";
import { TaskDialog } from "@/components/features/tasks/TaskDialog";
import { useToast } from "@/hooks/use-toast";

export default function CategoryDetails() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [category, setCategory] = useState<CategoryDetail | null>(null);
  const [tasks, setTasks] = useState<TaskMinimal[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [selectedTask, setSelectedTask] = useState<TaskDetail | null>(null);
  const { toast } = useToast();

  const fetchCategory = async () => {
    try {
      const { data } = await api.get<CategoryDetail>(`/categories/${id}`);
      setCategory(data);
    } catch (error) {
      console.error("Failed to fetch category", error);
      toast({ title: "Failed to fetch category", variant: "destructive" });
      navigate("/categories");
    }
  };

  const fetchTasks = async () => {
    try {
      const { data } = await api.get<PaginatedResponse<TaskMinimal>>(`/categories/${id}/tasks`, {
        params: { page: 0, page_size: 100 },
      });
      setTasks(data.items);
    } catch (error) {
      console.error("Failed to fetch tasks", error);
    }
  };

  useEffect(() => {
    if (id) {
      Promise.all([fetchCategory(), fetchTasks()]).finally(() => setIsLoading(false));
    }
  }, [id]);

  const handleCreate = async (values: any) => {
    try {
      await api.post(`/categories/${id}/tasks`, values);
      toast({ title: "Task created" });
      fetchTasks();
      fetchCategory(); // Update counts
    } catch (error) {
      toast({ title: "Failed to create task", variant: "destructive" });
    }
  };

  const handleUpdate = async (values: any) => {
    if (!selectedTask) return;
    try {
      await api.patch(`/categories/${id}/tasks/${selectedTask.id}`, values);
      toast({ title: "Task updated" });
      fetchTasks();
      fetchCategory();
    } catch (error) {
      toast({ title: "Failed to update task", variant: "destructive" });
    }
  };

  const handleDelete = async (taskId: number) => {
    try {
      await api.delete(`/categories/${id}/tasks/${taskId}`);
      toast({ title: "Task deleted" });
      fetchTasks();
      fetchCategory();
    } catch (error) {
      toast({ title: "Failed to delete task", variant: "destructive" });
    }
  };

  const openCreateDialog = () => {
    setSelectedTask(null);
    setIsDialogOpen(true);
  };

  const openEditDialog = async (task: TaskMinimal) => {
    try {
      const { data } = await api.get<TaskDetail>(`/categories/${id}/tasks/${task.id}`);
      setSelectedTask(data);
      setIsDialogOpen(true);
    } catch (error) {
      toast({ title: "Failed to fetch task details", variant: "destructive" });
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "done":
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case "cancel":
        return <XCircle className="h-4 w-4 text-red-500" />;
      default:
        return <Circle className="h-4 w-4 text-gray-500" />;
    }
  };

  if (isLoading) return <div>Loading...</div>;
  if (!category) return <div>Category not found</div>;

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" onClick={() => navigate("/categories")}>
          <ArrowLeft className="h-4 w-4" />
        </Button>
        <div>
          <h2 className="text-3xl font-bold tracking-tight">{category.name}</h2>
          <p className="text-muted-foreground">{category.description}</p>
        </div>
        <div className="ml-auto flex items-center gap-2">
          <Badge variant="secondary">{category.taskCount} Tasks</Badge>
          <Badge variant="outline" className="text-green-600 border-green-200 bg-green-50">
            {Math.round(category.progress * 100)}% Done
          </Badge>
        </div>
      </div>

      <div className="flex justify-end">
        <Button onClick={openCreateDialog}>
          <Plus className="mr-2 h-4 w-4" /> New Task
        </Button>
      </div>

      <div className="space-y-2">
        {tasks.map((task) => (
          <Card key={task.id} className="hover:bg-slate-50 transition-colors">
            <CardContent className="p-4 flex items-center gap-4">
              {getStatusIcon(task.status)}
              <div className="flex-1">
                <h3 className={`font-medium ${task.status === 'done' ? 'line-through text-muted-foreground' : ''}`}>
                  {task.title}
                </h3>
                {task.expiresAt && (
                  <p className="text-xs text-muted-foreground">
                    Due: {new Date(task.expiresAt).toLocaleDateString()}
                  </p>
                )}
              </div>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="icon">
                    <MoreVertical className="h-4 w-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={() => openEditDialog(task)}>
                    <Pencil className="mr-2 h-4 w-4" /> Edit
                  </DropdownMenuItem>
                  <DropdownMenuItem
                    className="text-red-600"
                    onClick={() => handleDelete(task.id)}
                  >
                    <Trash className="mr-2 h-4 w-4" /> Delete
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </CardContent>
          </Card>
        ))}
      </div>

      <TaskDialog
        open={isDialogOpen}
        onOpenChange={setIsDialogOpen}
        task={selectedTask}
        onSubmit={selectedTask ? handleUpdate : handleCreate}
      />
    </div>
  );
}
