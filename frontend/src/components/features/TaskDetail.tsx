import { useState, useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Calendar, RefreshCw, MessageSquare, Tag, ArrowLeft, Check, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import type { TaskDetailDto, TaskUpdateDto } from "@/types/task";

interface TaskDetailProps {
  task: TaskDetailDto;
  onUpdateField: (field: keyof TaskUpdateDto, value: any) => Promise<void>;
  onBack: () => void;
}

export function TaskDetail({ task, onUpdateField, onBack }: TaskDetailProps) {
  const [isEditingTitle, setIsEditingTitle] = useState(false);
  const [isEditingDescription, setIsEditingDescription] = useState(false);
  const [isEditingComment, setIsEditingComment] = useState(false);
  const [title, setTitle] = useState(task.title);
  const [description, setDescription] = useState(task.description || "");
  const [userComment, setUserComment] = useState(task.userComment || "");
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    setTitle(task.title);
    setDescription(task.description || "");
    setUserComment(task.userComment || "");
  }, [task]);

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

  const getReccurrenceLabel = (cycleTime: string | null) => {
    switch (cycleTime) {
      case '0 1 0': return 'Daily';
      case '0 7 0': return 'Weekly';
      case '1 0 0': return 'Monthly';
      default: return 'None';
    }
  };

  const formatDate = (dateString?: string | null | undefined) => {
    if (!dateString) return 'Not set';
    return new Date(dateString).toLocaleString("en-US", { 
      month: "short", 
      day: "numeric", 
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit"
    });
  };

  const handleSaveTitle = async () => {
    if (!title.trim() || title === task.title) {
      setIsEditingTitle(false);
      setTitle(task.title);
      return;
    }
    setIsSaving(true);
    try {
      await onUpdateField('title', title);
      setIsEditingTitle(false);
    } catch (err) {
      console.error("Failed to save title:", err);
    } finally {
      setIsSaving(false);
    }
  };

  const handleSaveDescription = async () => {
    setIsSaving(true);
    try {
      await onUpdateField('description', description);
      setIsEditingDescription(false);
    } catch (err) {
      console.error("Failed to save description:", err);
    } finally {
      setIsSaving(false);
    }
  };

  const handleSaveComment = async () => {
    setIsSaving(true);
    try {
      await onUpdateField('userComment', userComment);
      setIsEditingComment(false);
    } catch (err) {
      console.error("Failed to save comment:", err);
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="space-y-6 max-w-4xl mx-auto">
      <div className="flex items-center justify-between">
        <Button variant="ghost" className="gap-2" onClick={onBack}>
          <ArrowLeft className="w-4 h-4" /> Back
        </Button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-[1fr_300px] gap-6">
        <div className="space-y-6">
          <Card className="border-none shadow-md bg-white/50 backdrop-blur-sm">
            <CardHeader className="pb-4">
              <div className="flex items-start justify-between gap-4">
                <div className="space-y-1 flex-1">
                  {isEditingTitle ? (
                    <div className="flex items-center gap-2">
                       <Input 
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        className="text-2xl font-bold h-10 rounded-xl bg-muted/30 focus:bg-white transition-all px-3"
                        autoFocus
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') handleSaveTitle();
                          if (e.key === 'Escape') { setIsEditingTitle(false); setTitle(task.title); }
                        }}
                        disabled={isSaving}
                      />
                      <div className="flex gap-1 shrink-0">
                         <Button size="icon" variant="ghost" className="h-9 w-9 text-green-600 hover:text-green-700 hover:bg-green-50 rounded-full" onClick={handleSaveTitle} disabled={isSaving}>
                          <Check className="w-5 h-5" />
                        </Button>
                        <Button size="icon" variant="ghost" className="h-9 w-9 text-red-600 hover:text-red-700 hover:bg-red-50 rounded-full" onClick={() => { setIsEditingTitle(false); setTitle(task.title); }} disabled={isSaving}>
                          <X className="w-5 h-5" />
                        </Button>
                      </div>
                    </div>
                  ) : (
                    <CardTitle 
                      className="text-3xl font-bold tracking-tight cursor-pointer hover:text-primary/80 transition-colors"
                      onClick={() => setIsEditingTitle(true)}
                    >
                      {task.title}
                    </CardTitle>
                  )}
                  <div className="flex items-center gap-2 text-muted-foreground">
                    <Tag className="w-4 h-4" />
                    <span className="text-sm font-medium">{task.categoryName}</span>
                  </div>
                </div>
                
                <Select 
                  value={task.status} 
                  onValueChange={(value) => onUpdateField('status', value)}
                >
                  <SelectTrigger className={`w-36 ${getStatusColor(task.status)} border-none text-white font-semibold h-9 rounded-full shadow-sm`}>
                    <SelectValue>{getStatusLabel(task.status)}</SelectValue>
                  </SelectTrigger>
                  <SelectContent className="rounded-xl">
                    <SelectItem value="open">Open</SelectItem>
                    <SelectItem value="done">Done</SelectItem>
                    <SelectItem value="cancel">Cancel</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </CardHeader>
            <CardContent className="space-y-6">
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <h4 className="text-sm font-semibold uppercase tracking-wider text-muted-foreground">Description</h4>
                  {!isEditingDescription && (
                    <Button variant="ghost" size="sm" className="text-xs h-7" onClick={() => setIsEditingDescription(true)}>
                      Edit
                    </Button>
                  )}
                </div>
                
                {isEditingDescription ? (
                  <div className="space-y-3">
                    <Textarea 
                      value={description}
                      onChange={(e) => setDescription(e.target.value)}
                      className="min-h-[120px] rounded-xl bg-muted/30 focus:bg-white transition-all"
                      placeholder="Enter task description..."
                      disabled={isSaving}
                    />
                    <div className="flex justify-end gap-2">
                      <Button variant="ghost" size="sm" onClick={() => { setIsEditingDescription(false); setDescription(task.description || ""); }} disabled={isSaving}>
                        <X className="w-4 h-4 mr-1" /> Cancel
                      </Button>
                      <Button size="sm" onClick={handleSaveDescription} disabled={isSaving}>
                        <Check className="w-4 h-4 mr-1" /> Save
                      </Button>
                    </div>
                  </div>
                ) : (
                  <div className="bg-muted/30 p-4 rounded-xl text-foreground leading-relaxed cursor-pointer hover:bg-muted/50 transition-colors" onClick={() => setIsEditingDescription(true)}>
                    {task.description || "add description about this task"}
                  </div>
                )}
              </div>

              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <h4 className="text-sm font-semibold uppercase tracking-wider text-muted-foreground">User Comments</h4>
                  {!isEditingComment && (
                    <Button variant="ghost" size="sm" className="text-xs h-7" onClick={() => setIsEditingComment(true)}>
                      Edit
                    </Button>
                  )}
                </div>

                {isEditingComment ? (
                  <div className="space-y-3">
                    <Textarea 
                      value={userComment}
                      onChange={(e) => setUserComment(e.target.value)}
                      className="min-h-[80px] rounded-xl bg-muted/30 focus:bg-white transition-all"
                      placeholder="Add a comment..."
                      disabled={isSaving}
                    />
                    <div className="flex justify-end gap-2">
                      <Button variant="ghost" size="sm" onClick={() => { setIsEditingComment(false); setUserComment(task.userComment || ""); }} disabled={isSaving}>
                        <X className="w-4 h-4 mr-1" /> Cancel
                      </Button>
                      <Button size="sm" onClick={handleSaveComment} disabled={isSaving}>
                        <Check className="w-4 h-4 mr-1" /> Save
                      </Button>
                    </div>
                  </div>
                ) : (
                  <div className="bg-muted/30 p-4 rounded-xl text-foreground flex gap-3 items-start cursor-pointer hover:bg-muted/50 transition-colors" onClick={() => setIsEditingComment(true)}>
                    <MessageSquare className="w-5 h-5 mt-1 text-muted-foreground" />
                    <p>{task.userComment || "add comments about this task"}</p>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
        </div>

        <div className="space-y-6">
          <Card className="border-none shadow-sm bg-muted/20">
            <CardHeader>
              <CardTitle className="text-lg font-semibold">Schedule</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-1">
                <label className="text-xs font-medium text-muted-foreground flex items-center gap-1">
                  <Calendar className="w-3 h-3" /> Due Date
                </label>
                <p className="text-sm font-medium">{task.expiresAt ? formatDate(task.expiresAt) : "None"}</p>
              </div>
              
              <div className="space-y-1">
                <label className="text-xs font-medium text-muted-foreground flex items-center gap-1">
                  <RefreshCw className="w-3 h-3" /> Recurrence
                </label>
                <p className="text-sm font-medium">{getReccurrenceLabel(task.cycleTime!)}</p>
              </div>

              <div className="pt-4 border-t border-muted-foreground/10 space-y-3">
                <div className="space-y-1">
                  <label className="text-xs font-medium text-muted-foreground">Created at</label>
                  <p className="text-xs text-muted-foreground/80">{formatDate(task.createdAt)}</p>
                </div>
                <div className="space-y-1">
                  <label className="text-xs font-medium text-muted-foreground">Last updated</label>
                  <p className="text-xs text-muted-foreground/80">{formatDate(task.updatedAt)}</p>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
