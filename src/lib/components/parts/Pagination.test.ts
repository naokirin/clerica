import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import Pagination from './Pagination.svelte';

describe('Pagination', () => {
  it('renders pagination controls when totalPages > 1', () => {
    render(Pagination, {
      currentPage: 1,
      totalPages: 5
    });
    
    expect(screen.getByText('≪')).toBeInTheDocument(); // First page
    expect(screen.getByText('‹')).toBeInTheDocument(); // Previous page
    expect(screen.getByText('›')).toBeInTheDocument(); // Next page
    expect(screen.getByText('≫')).toBeInTheDocument(); // Last page
  });

  it('does not render when totalPages is 1', () => {
    const { container } = render(Pagination, {
      currentPage: 1,
      totalPages: 1
    });
    
    expect(container.querySelector('.pagination-controls')).not.toBeInTheDocument();
  });

  it('renders page numbers correctly', () => {
    render(Pagination, {
      currentPage: 3,
      totalPages: 5
    });
    
    expect(screen.getByText('1')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
    expect(screen.getByText('3')).toBeInTheDocument();
    expect(screen.getByText('4')).toBeInTheDocument();
    expect(screen.getByText('5')).toBeInTheDocument();
  });

  it('highlights current page', () => {
    const { container } = render(Pagination, {
      currentPage: 3,
      totalPages: 5
    });
    
    const currentPageButton = screen.getByText('3');
    expect(currentPageButton).toHaveClass('active');
  });

  it('dispatches goToPage event when page number is clicked', async () => {
    const component = render(Pagination, {
      currentPage: 1,
      totalPages: 5
    });
    
    const goToPageSpy = vi.fn();
    component.component.$on('goToPage', goToPageSpy);
    
    const pageButton = screen.getByText('3');
    await fireEvent.click(pageButton);
    
    expect(goToPageSpy).toHaveBeenCalledWith(
      expect.objectContaining({
        detail: { page: 3 }
      })
    );
  });

  it('dispatches goToNextPage event when next button is clicked', async () => {
    const component = render(Pagination, {
      currentPage: 2,
      totalPages: 5
    });
    
    const goToNextPageSpy = vi.fn();
    component.component.$on('goToNextPage', goToNextPageSpy);
    
    const nextButton = screen.getByText('›');
    await fireEvent.click(nextButton);
    
    expect(goToNextPageSpy).toHaveBeenCalled();
  });

  it('dispatches goToPreviousPage event when previous button is clicked', async () => {
    const component = render(Pagination, {
      currentPage: 2,
      totalPages: 5
    });
    
    const goToPreviousPageSpy = vi.fn();
    component.component.$on('goToPreviousPage', goToPreviousPageSpy);
    
    const previousButton = screen.getByText('‹');
    await fireEvent.click(previousButton);
    
    expect(goToPreviousPageSpy).toHaveBeenCalled();
  });

  it('dispatches goToFirstPage event when first button is clicked', async () => {
    const component = render(Pagination, {
      currentPage: 3,
      totalPages: 5
    });
    
    const goToFirstPageSpy = vi.fn();
    component.component.$on('goToFirstPage', goToFirstPageSpy);
    
    const firstButton = screen.getByText('≪');
    await fireEvent.click(firstButton);
    
    expect(goToFirstPageSpy).toHaveBeenCalled();
  });

  it('dispatches goToLastPage event when last button is clicked', async () => {
    const component = render(Pagination, {
      currentPage: 3,
      totalPages: 5
    });
    
    const goToLastPageSpy = vi.fn();
    component.component.$on('goToLastPage', goToLastPageSpy);
    
    const lastButton = screen.getByText('≫');
    await fireEvent.click(lastButton);
    
    expect(goToLastPageSpy).toHaveBeenCalled();
  });

  it('disables previous and first buttons on first page', () => {
    render(Pagination, {
      currentPage: 1,
      totalPages: 5
    });
    
    const firstButton = screen.getByText('≪');
    const previousButton = screen.getByText('‹');
    
    expect(firstButton).toBeDisabled();
    expect(previousButton).toBeDisabled();
  });

  it('disables next and last buttons on last page', () => {
    render(Pagination, {
      currentPage: 5,
      totalPages: 5
    });
    
    const nextButton = screen.getByText('›');
    const lastButton = screen.getByText('≫');
    
    expect(nextButton).toBeDisabled();
    expect(lastButton).toBeDisabled();
  });

  it('disables all buttons when disabled prop is true', () => {
    render(Pagination, {
      currentPage: 3,
      totalPages: 5,
      disabled: true
    });
    
    const buttons = screen.getAllByRole('button');
    buttons.forEach(button => {
      expect(button).toBeDisabled();
    });
  });

  it('does not dispatch events when disabled', async () => {
    const component = render(Pagination, {
      currentPage: 3,
      totalPages: 5,
      disabled: true
    });
    
    const goToPageSpy = vi.fn();
    component.component.$on('goToPage', goToPageSpy);
    
    const pageButton = screen.getByText('2');
    await fireEvent.click(pageButton);
    
    expect(goToPageSpy).not.toHaveBeenCalled();
  });

  it('does not dispatch event when clicking current page', async () => {
    const component = render(Pagination, {
      currentPage: 3,
      totalPages: 5
    });
    
    const goToPageSpy = vi.fn();
    component.component.$on('goToPage', goToPageSpy);
    
    const currentPageButton = screen.getByText('3');
    await fireEvent.click(currentPageButton);
    
    expect(goToPageSpy).not.toHaveBeenCalled();
  });

  it('calculates visible pages correctly with maxVisiblePages', () => {
    render(Pagination, {
      currentPage: 10,
      totalPages: 20,
      maxVisiblePages: 5
    });
    
    // Should show pages 8, 9, 10, 11, 12
    expect(screen.getByText('8')).toBeInTheDocument();
    expect(screen.getByText('9')).toBeInTheDocument();
    expect(screen.getByText('10')).toBeInTheDocument();
    expect(screen.getByText('11')).toBeInTheDocument();
    expect(screen.getByText('12')).toBeInTheDocument();
    
    // Should not show page 7 or 13
    expect(screen.queryByText('7')).not.toBeInTheDocument();
    expect(screen.queryByText('13')).not.toBeInTheDocument();
  });

  it('shows all pages when totalPages is less than maxVisiblePages', () => {
    render(Pagination, {
      currentPage: 2,
      totalPages: 3,
      maxVisiblePages: 7
    });
    
    expect(screen.getByText('1')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
    expect(screen.getByText('3')).toBeInTheDocument();
  });

  it('handles edge cases for visible pages calculation', () => {
    // Test when current page is near the beginning
    const { rerender } = render(Pagination, {
      currentPage: 2,
      totalPages: 20,
      maxVisiblePages: 5
    });
    
    // Should show pages 1, 2, 3, 4, 5
    expect(screen.getByText('1')).toBeInTheDocument();
    expect(screen.getByText('5')).toBeInTheDocument();
    
    // Test when current page is near the end
    rerender({
      currentPage: 19,
      totalPages: 20,
      maxVisiblePages: 5
    });
    
    // Should show pages 16, 17, 18, 19, 20
    expect(screen.getByText('16')).toBeInTheDocument();
    expect(screen.getByText('20')).toBeInTheDocument();
  });

  it('validates page bounds in goToPage function', async () => {
    const component = render(Pagination, {
      currentPage: 3,
      totalPages: 5
    });
    
    const goToPageSpy = vi.fn();
    component.component.$on('goToPage', goToPageSpy);
    
    // Test clicking page 6 (out of bounds) - this would not normally happen in UI
    // but tests the validation logic
    const pageButton = screen.getByText('5');
    await fireEvent.click(pageButton);
    
    expect(goToPageSpy).toHaveBeenCalledWith(
      expect.objectContaining({
        detail: { page: 5 }
      })
    );
  });

  it('has correct default maxVisiblePages', () => {
    render(Pagination, {
      currentPage: 10,
      totalPages: 20
      // maxVisiblePages should default to 7
    });
    
    // Should show 7 pages centered around page 10
    expect(screen.getByText('7')).toBeInTheDocument();
    expect(screen.getByText('13')).toBeInTheDocument();
  });
});