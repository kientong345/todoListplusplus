import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { CategoryCard } from "@/components/features/CategoryCard";
import type { CategoryMinimal } from "@/types";
import { Plus } from "lucide-react";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { CategoryForm } from "@/components/features/CategoryForm";


// Mock data for initial rendering and testing
const MOCK_CATEGORIES: CategoryMinimal[] = [
  { id: "1", userId: "u1", name: "Work", taskCount: 12, imageUrl: null },
  { id: "2", userId: "u1", name: "Home", taskCount: 8, imageUrl: null },
  { id: "3", userId: "u1", name: "Groceries", taskCount: 5, imageUrl: null },
  { id: "4", userId: "u1", name: "Study", taskCount: 3, imageUrl: null },
  { id: "5", userId: "u1", name: "Fitness", taskCount: 7, imageUrl: null },
  { id: "6", userId: "u1", name: "Travel", taskCount: 2, imageUrl: null },
];

export default function DashboardPage() {
  const [categories, setCategories] = useState<CategoryMinimal[]>(MOCK_CATEGORIES);
  const [isLoading, setIsLoading] = useState(false);
  const [isAddDialogOpen, setIsAddDialogOpen] = useState(false);

  useEffect(() => {
    // TODO: Fetch categories from API
    // setIsLoading(true);
    // api.get('/categories').then(...)
  }, [setCategories, setIsLoading]);


  const handleAddCategory = (data: { name: string; imageUrl: string; description: string }) => {
    setIsLoading(true);
    // TODO: Implement actual API call
    console.log("Adding category:", data);
    setTimeout(() => {
      const newCategory: CategoryMinimal = {
        id: Math.random().toString(36).substring(2, 9),
        userId: "u1",
        name: data.name,
        imageUrl: data.imageUrl || null,
        description: data.description || null,
        taskCount: 0,
      };
      setCategories([...categories, newCategory]);
      setIsLoading(false);
      setIsAddDialogOpen(false);
    }, 1000);
  };

  const handleUpdateCategory = (id: string, data: { name: string; imageUrl: string; description: string }) => {
    setCategories(categories.map(cat => 
      cat.id === id ? { ...cat, ...data, imageUrl: data.imageUrl || null, description: data.description || null } : cat
    ));
  };

  const handleDeleteCategory = (id: string) => {
    setCategories(categories.filter(cat => cat.id !== id));
  };

  return (
    <div className="space-y-8">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-3xl font-extrabold tracking-tight">My Categories</h1>
          <p className="text-muted-foreground mt-1">
            Organize and manage your tasks by category.
          </p>
        </div>
        <Dialog open={isAddDialogOpen} onOpenChange={setIsAddDialogOpen}>
          <DialogTrigger asChild>
            <Button className="font-semibold shadow-sm w-full sm:w-auto">
              <Plus className="mr-2 h-4 w-4" /> Add Category
            </Button>
          </DialogTrigger>
          <DialogContent className="sm:max-w-[425px] rounded-2xl">
            <DialogHeader>
              <DialogTitle className="text-2xl font-bold">Add Category</DialogTitle>
              <DialogDescription>
                Create a new workspace for your tasks.
              </DialogDescription>
            </DialogHeader>
            <CategoryForm 
              onSubmit={handleAddCategory} 
              onCancel={() => setIsAddDialogOpen(false)}
              isLoading={isLoading}
              submitLabel="Create Category"
            />
          </DialogContent>
        </Dialog>
      </div>

      {isLoading && !isAddDialogOpen ? (
        <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
          {[...Array(4)].map((_, i) => (
            <div key={i} className="h-48 rounded-xl bg-muted animate-pulse" />
          ))}
        </div>
      ) : categories.length > 0 ? (
        <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
          {categories.map((category) => (
            <CategoryCard 
              key={category.id} 
              category={category} 
              onUpdate={handleUpdateCategory}
              onDelete={handleDeleteCategory}
            />
          ))}
        </div>
      ) : (

        <div className="text-center py-20 border-2 border-dashed rounded-2xl bg-muted/30">
          <p className="text-muted-foreground">No categories found. Create your first one!</p>
          <Button variant="link" className="mt-2">
            Add Category
          </Button>
        </div>
      )}
    </div>
  );
}
