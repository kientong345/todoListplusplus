import { useState, useEffect } from "react";
import { api } from "@/lib/api";
import type { CategoryMinimal } from "@/types/category";
import type { PaginatedResponse } from "@/types/common";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Plus, MoreVertical, Pencil, Trash } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { CategoryDialog } from "@/components/features/categories/CategoryDialog";
import { useToast } from "@/hooks/use-toast";

export default function Categories() {
  const [categories, setCategories] = useState<CategoryMinimal[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [selectedCategory, setSelectedCategory] = useState<CategoryMinimal | null>(null);
  const { toast } = useToast();

  const fetchCategories = async () => {
    try {
      const { data } = await api.get<PaginatedResponse<CategoryMinimal>>("/categories", {
        params: { page: 1, pageSize: 20, sortBy: 'new-update' },
      });
      setCategories(data.items);
    } catch (error) {
      console.error("Failed to fetch categories", error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchCategories();
  }, []);

  const handleCreate = async (values: any) => {
    try {
      await api.post("/categories", values);
      toast({ title: "Category created" });
      fetchCategories();
    } catch (error) {
      toast({ title: "Failed to create category", variant: "destructive" });
    }
  };

  const handleUpdate = async (values: any) => {
    if (!selectedCategory) return;
    try {
      await api.patch(`/categories/${selectedCategory.id}`, values);
      toast({ title: "Category updated" });
      fetchCategories();
    } catch (error) {
      toast({ title: "Failed to update category", variant: "destructive" });
    }
  };

  const handleDelete = async (id: number) => {
    try {
      await api.delete(`/categories/${id}`);
      toast({ title: "Category deleted" });
      fetchCategories();
    } catch (error) {
      toast({ title: "Failed to delete category", variant: "destructive" });
    }
  };

  const openCreateDialog = () => {
    setSelectedCategory(null);
    setIsDialogOpen(true);
  };

  const openEditDialog = (category: CategoryMinimal) => {
    setSelectedCategory(category);
    setIsDialogOpen(true);
  };

  if (isLoading) return <div>Loading...</div>;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-3xl font-bold tracking-tight">Categories</h2>
        <Button onClick={openCreateDialog}>
          <Plus className="mr-2 h-4 w-4" /> New Category
        </Button>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {categories.map((category) => (
          <Card key={category.id} className="hover:shadow-md transition-shadow">
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-xl font-semibold">
                {category.name}
              </CardTitle>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" className="h-8 w-8 p-0">
                    <MoreVertical className="h-4 w-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={() => openEditDialog(category)}>
                    <Pencil className="mr-2 h-4 w-4" /> Edit
                  </DropdownMenuItem>
                  <DropdownMenuItem
                    className="text-red-600"
                    onClick={() => handleDelete(category.id)}
                  >
                    <Trash className="mr-2 h-4 w-4" /> Delete
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </CardHeader>
            <CardContent>
              {category.description && (
                <CardDescription className="line-clamp-2 mb-4">
                  {category.description}
                </CardDescription>
              )}
              <div className="text-sm text-muted-foreground">
                {category.taskCount} tasks
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      <CategoryDialog
        open={isDialogOpen}
        onOpenChange={setIsDialogOpen}
        category={selectedCategory}
        onSubmit={selectedCategory ? handleUpdate : handleCreate}
      />
    </div>
  );
}
