import { useState, useEffect } from "react";
import { useParams, Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { CategoryDetail } from "@/components/features/CategoryDetail";
import { TaskList } from "@/components/features/TaskList";
import { ArrowLeft, Plus } from "lucide-react";
import type { Task } from "@/types";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { TaskForm } from "@/components/features/TaskForm";

// Mock Data
const MOCK_CATEGORY_DETAIL = {
  id: "1",
  name: "Work Project",
  description: "Tasks related to the Q4 product launch and marketing campaign.",
  imageUrl: null,
  taskCount: 12,
  openedTaskCount: 5,
  canceledTaskCount: 1,
  doneTaskCount: 6,
  progress: 50,
};

const MOCK_TASKS: Task[] = [
  { id: "101", categoryId: "1", userId: "u1", title: "Review Q4 Marketing Plan", description: "Analyze the budget and channels.", status: "open", expiresAt: "2023-10-28", cycleTime: "weekly", createdAt: "", updatedAt: "" },
  { id: "102", categoryId: "1", userId: "u1", title: "Finalize Budget Proposal", description: "Get sign-off from finance.", status: "open", expiresAt: "2023-10-30", createdAt: "", updatedAt: "" },
  { id: "103", categoryId: "1", userId: "u1", title: "Team Sync Meeting", status: "done", expiresAt: "2023-10-25", cycleTime: "daily", createdAt: "", updatedAt: "" },
  { id: "104", categoryId: "1", userId: "u1", title: "Update Stakeholders", status: "open", expiresAt: "2023-11-01", createdAt: "", updatedAt: "" },
  { id: "105", categoryId: "1", userId: "u1", title: "Draft Launch Email", status: "open", expiresAt: "2023-11-05", createdAt: "", updatedAt: "" },
  { id: "106", categoryId: "1", userId: "u1", title: "Cancelled Task Example", status: "canceled", expiresAt: "2023-10-20", createdAt: "", updatedAt: "" },
];

export default function CategoryPage() {
  const { id } = useParams();
  const [tasks, setTasks] = useState<Task[]>(MOCK_TASKS);
  const [isLoading, setIsLoading] = useState(false);
  const [isAddTaskDialogOpen, setIsAddTaskDialogOpen] = useState(false);

  useEffect(() => {
    // TODO: Fetch category details and tasks
    setIsLoading(true);
    setTimeout(() => setIsLoading(false), 500);
  }, [id]);

  const handleToggleStatus = (taskId: string) => {
    setTasks(tasks.map(t => 
      t.id === taskId 
        ? { ...t, status: t.status === 'done' ? 'open' : 'done' } 
        : t
    ));
  };

  const handleAddTask = (data: Partial<Task>) => {
    setIsLoading(true);
    // TODO: Connect to API
    setTimeout(() => {
      const newTask: Task = {
        id: Math.random().toString(36).substr(2, 9),
        categoryId: id || "1",
        userId: "u1",
        title: data.title!,
        description: data.description,
        status: "open",
        cycleTime: data.cycleTime || null,
        expiresAt: data.expiresAt || null,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      setTasks([newTask, ...tasks]);
      setIsLoading(false);
      setIsAddTaskDialogOpen(false);
    }, 800);
  };


  const handleUpdateTask = (taskId: string, data: Partial<Task>) => {
    setTasks(tasks.map(t => 
      t.id === taskId ? { ...t, ...data } : t
    ));
  };

  const handleDeleteTask = (taskId: string) => {
    setTasks(tasks.filter(t => t.id !== taskId));
  };

  return (
    <div className="space-y-6 max-w-5xl mx-auto">
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" asChild>
          <Link to="/"><ArrowLeft className="w-5 h-5" /></Link>
        </Button>
        <h2 className="text-lg font-semibold text-muted-foreground">Back to Dashboard</h2>
      </div>

      <CategoryDetail category={MOCK_CATEGORY_DETAIL} />

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
              <TaskForm
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
          onUpdate={handleUpdateTask}
          onDelete={handleDeleteTask}
        />
      </div>
    </div>
  );
}


