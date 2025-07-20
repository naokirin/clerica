import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import LoadingScreen from './LoadingScreen.svelte';
import type { LoadingSteps } from '../../types';

// Mock i18n
vi.mock('$lib/i18n', () => ({
  t: vi.fn((key: string) => key)
}));

const createMockSteps = (overrides: Partial<LoadingSteps> = {}): LoadingSteps => ({
  directories: false,
  tags: false,
  files: false,
  ...overrides
});

describe('LoadingScreen', () => {
  it('renders nothing when not visible', () => {
    const { container } = render(LoadingScreen, {
      isVisible: false,
      progress: 50,
      steps: createMockSteps()
    });
    
    expect(container.querySelector('.loading-overlay')).not.toBeInTheDocument();
  });

  it('renders loading screen when visible', () => {
    render(LoadingScreen, {
      isVisible: true,
      progress: 50,
      steps: createMockSteps()
    });
    
    expect(screen.getByText('Clerica')).toBeInTheDocument();
    expect(screen.getByText('common.app.description')).toBeInTheDocument();
  });

  it('displays progress percentage', () => {
    render(LoadingScreen, {
      isVisible: true,
      progress: 75,
      steps: createMockSteps()
    });
    
    expect(screen.getByText('75%')).toBeInTheDocument();
  });

  it('shows spinner when progress is less than 100', () => {
    const { container } = render(LoadingScreen, {
      isVisible: true,
      progress: 50,
      steps: createMockSteps()
    });
    
    expect(container.querySelector('.loading-spinner')).toBeInTheDocument();
    expect(container.querySelector('.loading-complete')).not.toBeInTheDocument();
  });

  it('shows completion message when progress is 100', () => {
    render(LoadingScreen, {
      isVisible: true,
      progress: 100,
      steps: createMockSteps()
    });
    
    expect(screen.getByText('common.loading.ready')).toBeInTheDocument();
    expect(screen.queryByRole('img')).not.toBeInTheDocument(); // No spinner
  });

  it('sets progress bar width correctly', () => {
    const { container } = render(LoadingScreen, {
      isVisible: true,
      progress: 60,
      steps: createMockSteps()
    });
    
    const progressFill = container.querySelector('.progress-fill');
    expect(progressFill).toHaveStyle('width: 60%');
  });

  it('displays all loading steps', () => {
    render(LoadingScreen, {
      isVisible: true,
      progress: 50,
      steps: createMockSteps()
    });
    
    expect(screen.getByText('common.loading.directoriesLoading')).toBeInTheDocument();
    expect(screen.getByText('common.loading.tagsLoading')).toBeInTheDocument();
    expect(screen.getByText('common.loading.filesLoading')).toBeInTheDocument();
  });

  it('shows completed state for directories step', () => {
    const { container } = render(LoadingScreen, {
      isVisible: true,
      progress: 33,
      steps: createMockSteps({ directories: true })
    });
    
    expect(screen.getByText('common.loading.directoriesComplete')).toBeInTheDocument();
    
    const directoriesStep = container.querySelector('.loading-step.completed');
    expect(directoriesStep).toBeInTheDocument();
    expect(directoriesStep).toHaveTextContent('✓');
  });

  it('shows completed state for tags step', () => {
    render(LoadingScreen, {
      isVisible: true,
      progress: 66,
      steps: createMockSteps({ directories: true, tags: true })
    });
    
    expect(screen.getByText('common.loading.tagsComplete')).toBeInTheDocument();
    expect(screen.getByText('common.loading.directoriesComplete')).toBeInTheDocument();
  });

  it('shows completed state for files step', () => {
    render(LoadingScreen, {
      isVisible: true,
      progress: 100,
      steps: createMockSteps({ directories: true, tags: true, files: true })
    });
    
    expect(screen.getByText('common.loading.filesComplete')).toBeInTheDocument();
  });

  it('shows dots for incomplete steps', () => {
    const { container } = render(LoadingScreen, {
      isVisible: true,
      progress: 10,
      steps: createMockSteps()
    });
    
    const stepDots = container.querySelectorAll('.step-dot');
    expect(stepDots).toHaveLength(3); // All three steps should have dots
  });

  it('shows checkmarks for completed steps only', () => {
    render(LoadingScreen, {
      isVisible: true,
      progress: 66,
      steps: createMockSteps({ directories: true, tags: true })
    });
    
    // Should have 2 checkmarks and 1 dot
    const checkmarks = screen.getAllByText('✓');
    expect(checkmarks).toHaveLength(2);
  });

  it('applies completed class to finished steps', () => {
    const { container } = render(LoadingScreen, {
      isVisible: true,
      progress: 33,
      steps: createMockSteps({ directories: true })
    });
    
    const completedSteps = container.querySelectorAll('.loading-step.completed');
    expect(completedSteps).toHaveLength(1);
  });

  it('handles all steps completed', () => {
    const { container } = render(LoadingScreen, {
      isVisible: true,
      progress: 100,
      steps: createMockSteps({ directories: true, tags: true, files: true })
    });
    
    const completedSteps = container.querySelectorAll('.loading-step.completed');
    expect(completedSteps).toHaveLength(3);
    
    const checkmarks = screen.getAllByText('✓');
    expect(checkmarks).toHaveLength(3);
  });

  it('renders with zero progress', () => {
    const { container } = render(LoadingScreen, {
      isVisible: true,
      progress: 0,
      steps: createMockSteps()
    });
    
    expect(screen.getByText('0%')).toBeInTheDocument();
    
    const progressFill = container.querySelector('.progress-fill');
    expect(progressFill).toHaveStyle('width: 0%');
  });

  it('has correct overlay structure', () => {
    const { container } = render(LoadingScreen, {
      isVisible: true,
      progress: 50,
      steps: createMockSteps()
    });
    
    expect(container.querySelector('.loading-overlay')).toBeInTheDocument();
    expect(container.querySelector('.loading-container')).toBeInTheDocument();
    expect(container.querySelector('.loading-logo')).toBeInTheDocument();
    expect(container.querySelector('.loading-content')).toBeInTheDocument();
  });

  it('displays app title and description', () => {
    render(LoadingScreen, {
      isVisible: true,
      progress: 50,
      steps: createMockSteps()
    });
    
    expect(screen.getByRole('heading', { name: 'Clerica' })).toBeInTheDocument();
    expect(screen.getByText('common.app.description')).toBeInTheDocument();
  });

  it('has proper step icon structure', () => {
    const { container } = render(LoadingScreen, {
      isVisible: true,
      progress: 33,
      steps: createMockSteps({ directories: true })
    });
    
    const stepIcons = container.querySelectorAll('.step-icon');
    expect(stepIcons).toHaveLength(3);
    
    // First step should have checkmark, others should have dots
    const firstStepIcon = stepIcons[0];
    expect(firstStepIcon).toHaveTextContent('✓');
    
    const secondStepIcon = stepIcons[1];
    expect(secondStepIcon.querySelector('.step-dot')).toBeInTheDocument();
  });
});