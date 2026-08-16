import { describe, expect, it } from 'vitest'
import { drainSssfEvents, mapSssfTrace, type SssfApi, type SssfEventsPage } from './sssf'
import { sssfParityBinding, sssfParityDetail, sssfParityEvents } from './fixtures/sssf-parity'

describe('SSSF source adapter', () => {
  it('drains cursor pages sequentially without dropping insertion order', async () => {
    const pages: Record<number, SssfEventsPage> = {
      0: { events: sssfParityEvents.slice(0, 2), cursor: 2, has_more: true },
      2: { events: sssfParityEvents.slice(2, 5), cursor: 5, has_more: true },
      5: { events: sssfParityEvents.slice(5), cursor: 7, has_more: false },
    }
    const calls: number[] = []
    const api: SssfApi = {
      async session() { return sssfParityDetail },
      async events(_id, after) { calls.push(after); return pages[after]! },
    }
    const result = await drainSssfEvents(api, 'adw-parity-001')
    expect(calls).toEqual([0, 2, 5])
    expect(result.cursor).toBe(7)
    expect(result.events.map((event) => event.rowid)).toEqual([1, 2, 3, 4, 5, 6, 7])
  })

  it('preserves canonical Factory refs while retaining SSSF native ids', () => {
    const trace = mapSssfTrace(sssfParityDetail, sssfParityEvents, sssfParityBinding)
    expect(trace.runRef).toBe('run:01ARZ3NDEKTSV4RRFFQ69G5FAB')
    expect(trace.executionRef).toBe('execution:01ARZ3NDEKTSV4RRFFQ69G5FAG')
    expect(trace.nativeTrajectory?.ref).toBe('sssf:adw-parity-001')
    expect(trace.spans[1]?.nativeSpanRef).toBe('p-plan')
    expect(trace.spans[1]?.events[1]?.nativeRef).toBe('e-read')
  })

  it('maps failed tool detail and does not fabricate process events', () => {
    const trace = mapSssfTrace(sssfParityDetail, sssfParityEvents, sssfParityBinding)
    const failed = trace.spans.flatMap((span) => span.events).find((event) => event.nativeRef === 'e-test')
    expect(failed?.toolCall?.tool).toBe('bash')
    expect(failed?.toolCall?.ok).toBe(false)
    expect(failed?.toolCall?.durationMs).toBe(3100)
    expect(failed?.status).toBe('fail')
    expect(trace.spans.flatMap((span) => span.events).some((event) => event.kind === 'process')).toBe(false)
  })
})
