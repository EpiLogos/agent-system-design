import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { sssfParityTrace } from '../fixtures/sssf-parity'
import { ExecutionTraceExplorer } from './ExecutionTraceExplorer'

describe('SSSF-derived execution explorer', () => {
  it('renders sessions, waterfall, selected phase and scan-friendly stats', () => {
    render(<ExecutionTraceExplorer traces={[sssfParityTrace]} />)
    expect(screen.getByText('Sessions')).toBeTruthy()
    expect(screen.getAllByText('Preserve source-faithful execution detail while porting the Build surface.').length).toBeGreaterThanOrEqual(2)
    expect(screen.getByLabelText('Execution trace waterfall')).toBeTruthy()
    expect(screen.getByLabelText('Phase detail: request')).toBeTruthy()
    expect(screen.getByText('18,432 tokens')).toBeTruthy()
  })

  it('moves phase selection from the waterfall with arrow keys', () => {
    render(<ExecutionTraceExplorer traces={[sssfParityTrace]} />)
    const waterfall = screen.getByLabelText('Execution trace waterfall')
    fireEvent.keyDown(waterfall, { key: 'ArrowRight' })
    expect(screen.getByLabelText('Phase detail: plan')).toBeTruthy()
  })

  it('keeps failed tool args/results/duration deeply inspectable', () => {
    render(<ExecutionTraceExplorer traces={[sssfParityTrace]} />)
    const testPhase = screen.getAllByTitle('test — fail').find((element) => element.tagName === 'BUTTON')
    if (!testPhase) throw new Error('test phase button missing')
    fireEvent.click(testPhase)
    const row = screen.getByText('bash').closest('summary')
    expect(row?.textContent).toContain('3s')
    expect(row?.textContent).toContain('failed')
    if (!row) throw new Error('tool row missing')
    fireEvent.click(row)
    expect(screen.getByText(/npm test/)).toBeTruthy()
    expect(screen.getByText(/1 failed, 12 passed/)).toBeTruthy()
    expect(screen.getByText('e-test')).toBeTruthy()
  })

  it('does not render a process section when the pinned SSSF source supplied none', () => {
    render(<ExecutionTraceExplorer traces={[sssfParityTrace]} />)
    expect(screen.queryByText(/process\/native events/)).toBeNull()
  })
})
