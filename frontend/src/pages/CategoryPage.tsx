import { useState, useEffect } from "react";
import { useParams, Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { CategoryDetail } from "@/components/features/CategoryDetail";
import { TaskList } from "@/components/features/TaskList";
import { ArrowLeft, Plus, Loader2 } from "lucide-react";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { TaskCreateForm } from "@/components/features/TaskForm";
import { categories as categoryService, tasks as taskService } from "@/services/api";
import type { TaskCreateDto } from "@/types/task";
import type { CategoryDetailDto } from "@/types/category";
import type { TaskMinimalDto } from "@/types/task";

export default function CategoryPage() {
  const { id } = useParams<{ id: string }>();
  
  const [category, setCategory] = useState<CategoryDetailDto>();
  const [tasks, setTasks] = useState<TaskMinimalDto[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isAddTaskDialogOpen, setIsAddTaskDialogOpen] = useState(false);

  // Default pagination params as requested (page=1 logic handled as index 1)
  const [page] = useState(1); 

  const PAGE_SIZE = 20;

  const fetchData = async () => {
    if (!id) return;
    setIsLoading(true);
    try {
      const [categoryData, tasksData] = await Promise.all([
        categoryService.getOne(id),
        taskService.getAll(id, { page, pageSize: PAGE_SIZE, status: [], titlePattern: '', sortBy: 'latest' })
      ]);
      setCategory(categoryData);
      setTasks(tasksData.items);
    } catch (error) {
      console.error("Failed to fetch data:", error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, [id, page]);

  const handleToggleStatus = async (taskId: string) => {
    const task = tasks.find(t => t.id === taskId);
    if (!task || !id) return;
    
    // Toggle logic: done -> open, open/in_progress -> done
    const newStatus = task.status === 'done' ? 'open' : 'done';
    
    // Optimistic update
    setTasks(tasks.map(t => t.id === taskId ? { ...t, status: newStatus } : t));
    
    try {
      await taskService.update(id, taskId, { status: newStatus });
      // Refresh category stats if needed, or handle locally
      fetchData(); // Simplest way to sync counters
    } catch (error) {
      console.error("Failed to update status:", error);
      fetchData(); // Revert on error
    }
  };

  const handleAddTask = async (data: TaskCreateDto) => {
    if (!id) return;
    setIsLoading(true);
    try {
      await taskService.create(id, data);
      await fetchData();
      setIsAddTaskDialogOpen(false);
    } catch (error) {
       console.error("Failed to create task:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleDeleteTask = async (taskId: string) => {
    if (!id) return;
    try {
      await taskService.delete(id, taskId);
      await fetchData();
    } catch (error) {
      console.error("Failed to delete task:", error);
    }
  };

  if (isLoading && !category) {
    return (
      <div className="flex h-[50vh] items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    );
  }

  if (!category) {
    return (
      <div className="text-center py-20">
         <h2 className="text-xl font-semibold">Category not found</h2>
         <Button asChild variant="link" className="mt-4"><Link to="/">Go back home</Link></Button>
      </div>
    );
  }

  return (
    <div className="space-y-6 max-w-5xl mx-auto">
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" asChild>
          <Link to="/"><ArrowLeft className="w-5 h-5" /></Link>
        </Button>
        <h2 className="text-lg font-semibold text-muted-foreground text-foreground">Back to Dashboard</h2>
      </div>

      <CategoryDetail category={category} />

      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h3 className="text-2xl font-bold tracking-tight">Tasks</h3>
          
          <Dialog open={isAddTaskDialogOpen} onOpenChange={setIsAddTaskDialogOpen}>
            <DialogTrigger asChild>
              <Button>
                <Plus className="mr-2 h-4 w-4" /> Add Task
              </Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[500px] rounded-2xl">
              <DialogHeader>
                <DialogTitle className="text-2xl font-bold">Add Task</DialogTitle>
                <DialogDescription>
                  Create a new task in this category.
                </DialogDescription>
              </DialogHeader>
              <TaskCreateForm
                onSubmit={handleAddTask}
                onCancel={() => setIsAddTaskDialogOpen(false)}
                submitLabel="Create Task"
                isLoading={isLoading}
              />
            </DialogContent>
          </Dialog>
        </div>
        
        <TaskList 
          tasks={tasks} 
          onToggleStatus={handleToggleStatus} 
          onDelete={handleDeleteTask}
        />
      </div>
    </div>
  );
}



