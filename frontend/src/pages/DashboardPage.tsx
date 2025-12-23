import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { CategoryCard } from "@/components/features/CategoryCard";
import type { CategoryMinimal } from "@/types";
import { Plus } from "lucide-react";

import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { CategoryForm } from "@/components/features/CategoryForm";
import { categories as categoryService } from "@/services/api";

export default function DashboardPage() {
  const [categories, setCategories] = useState<CategoryMinimal[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isAddDialogOpen, setIsAddDialogOpen] = useState(false);

  const fetchCategories = async () => {
    setIsLoading(true);
    try {
      const response = await categoryService.getAll(1, 20, '', 'new-update');
      setCategories(response.items);
    } catch (error) {
      console.error("Failed to fetch categories:", error);
      // Optional: Add toast error handling here
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchCategories();
  }, []);

  const handleAddCategory = async (data: { name: string; imageUrl: string; description: string }) => {
    setIsLoading(true);
    try {
      await categoryService.create(data);
      await fetchCategories(); // Refresh list
      setIsAddDialogOpen(false);
    } catch (error) {
      console.error("Failed to create category:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleUpdateCategory = async (id: string, data: { name: string; imageUrl: string; description: string }) => {
    try {
      await categoryService.update(id, data);
      setCategories(categories.map(cat => 
        cat.id === id ? { ...cat, ...data, imageUrl: data.imageUrl || null, description: data.description || null } : cat
      ));
    } catch (error) {
      console.error("Failed to update category:", error);
    }
  };

  const handleDeleteCategory = async (id: string) => {
    try {
       await categoryService.delete(id);
       setCategories(categories.filter(cat => cat.id !== id));
    } catch (error) {
      console.error("Failed to delete category:", error);
    }
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

      {isLoading && categories.length === 0 ? (
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
          <Button variant="link" className="mt-2" onClick={() => setIsAddDialogOpen(true)}>
            Add Category
          </Button>
        </div>
      )}
    </div>
  );
}

