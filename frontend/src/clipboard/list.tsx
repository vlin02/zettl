import { useEffect, useRef } from 'react'
import type { CSSProperties, ReactNode } from 'react'
import { List, AutoSizer, CellMeasurer, CellMeasurerCache } from 'react-virtualized'

type Props<T> = {
  items: T[]
  selectedIndex: number
  renderItem: (item: T, isSelected: boolean, index: number) => ReactNode
  onLoadMore: () => void
}

export function VirtualizedList<T>({ items, selectedIndex, renderItem, onLoadMore }: Props<T>) {
  const listRef = useRef<List>(null)
  const cache = useRef(
    new CellMeasurerCache({
      fixedWidth: true,
    }),
  )

  const renderRow = ({ index, key, style, parent }: { index: number; key: string; style: CSSProperties; parent: any }) => (
    <CellMeasurer key={key} cache={cache.current} parent={parent} columnIndex={0} rowIndex={index}>
      {({ registerChild }) => (
        <div ref={registerChild} style={style}>
          {renderItem(items[index], selectedIndex === index, index)}
        </div>
      )}
    </CellMeasurer>
  )

  useEffect(() => {
    if (selectedIndex !== -1 && listRef.current) {
      listRef.current.scrollToRow(selectedIndex)
    }
  }, [selectedIndex])

  return (
    <AutoSizer>
      {({ height, width }) => (
        <List
          ref={listRef}
          height={height}
          width={width}
          rowCount={items.length}
          rowHeight={cache.current.rowHeight}
          deferredMeasurementCache={cache.current}
          rowRenderer={renderRow}
          onScroll={({ scrollTop, scrollHeight, clientHeight }) => {
            const dist = scrollHeight - scrollTop - clientHeight
            if (dist <= clientHeight * 2) onLoadMore()
          }}
          overscanRowCount={5}
        />
      )}
    </AutoSizer>
  )
}

