/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QtCore/QObject>
#include <QtCore/QAbstractItemModel>

class Demo;
class Fibonacci;
class FibonacciList;
class FileSystemTree;
class Processes;
class TimeSeries;

class Demo : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Fibonacci* const m_fibonacci;
    FibonacciList* const m_fibonacciList;
    FileSystemTree* const m_fileSystemTree;
    Processes* const m_processes;
    TimeSeries* const m_timeSeries;
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(Fibonacci* fibonacci READ fibonacci NOTIFY fibonacciChanged FINAL)
    Q_PROPERTY(FibonacciList* fibonacciList READ fibonacciList NOTIFY fibonacciListChanged FINAL)
    Q_PROPERTY(FileSystemTree* fileSystemTree READ fileSystemTree NOTIFY fileSystemTreeChanged FINAL)
    Q_PROPERTY(Processes* processes READ processes NOTIFY processesChanged FINAL)
    Q_PROPERTY(TimeSeries* timeSeries READ timeSeries NOTIFY timeSeriesChanged FINAL)
    explicit Demo(bool owned, QObject *parent);
public:
    explicit Demo(QObject *parent = nullptr);
    ~Demo();
    const Fibonacci* fibonacci() const;
    Fibonacci* fibonacci();
    const FibonacciList* fibonacciList() const;
    FibonacciList* fibonacciList();
    const FileSystemTree* fileSystemTree() const;
    FileSystemTree* fileSystemTree();
    const Processes* processes() const;
    Processes* processes();
    const TimeSeries* timeSeries() const;
    TimeSeries* timeSeries();
Q_SIGNALS:
    void fibonacciChanged();
    void fibonacciListChanged();
    void fileSystemTreeChanged();
    void processesChanged();
    void timeSeriesChanged();
};

class Fibonacci : public QObject
{
    Q_OBJECT
    friend class Demo;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(quint32 input READ input WRITE setInput NOTIFY inputChanged FINAL)
    Q_PROPERTY(quint64 result READ result NOTIFY resultChanged FINAL)
    explicit Fibonacci(bool owned, QObject *parent);
public:
    explicit Fibonacci(QObject *parent = nullptr);
    ~Fibonacci();
    quint32 input() const;
    void setInput(quint32 v);
    quint64 result() const;
Q_SIGNALS:
    void inputChanged();
    void resultChanged();
};

class FibonacciList : public QAbstractItemModel
{
    Q_OBJECT
    friend class Demo;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    explicit FibonacciList(bool owned, QObject *parent);
public:
    explicit FibonacciList(QObject *parent = nullptr);
    ~FibonacciList();

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE quint64 fibonacciNumber(int row) const;
    Q_INVOKABLE quint64 row(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
};

class FileSystemTree : public QAbstractItemModel
{
    Q_OBJECT
    friend class Demo;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(QString path READ path WRITE setPath NOTIFY pathChanged FINAL)
    explicit FileSystemTree(bool owned, QObject *parent);
public:
    explicit FileSystemTree(QObject *parent = nullptr);
    ~FileSystemTree();
    QString path() const;
    void setPath(const QString& v);

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE QByteArray fileIcon(const QModelIndex& index) const;
    Q_INVOKABLE QString fileName(const QModelIndex& index) const;
    Q_INVOKABLE QString filePath(const QModelIndex& index) const;
    Q_INVOKABLE qint32 filePermissions(const QModelIndex& index) const;
    Q_INVOKABLE QVariant fileSize(const QModelIndex& index) const;
    Q_INVOKABLE qint32 fileType(const QModelIndex& index) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
    void pathChanged();
};

class Processes : public QAbstractItemModel
{
    Q_OBJECT
    friend class Demo;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(bool active READ active WRITE setActive NOTIFY activeChanged FINAL)
    explicit Processes(bool owned, QObject *parent);
public:
    explicit Processes(QObject *parent = nullptr);
    ~Processes();
    bool active() const;
    void setActive(bool v);

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE QString cmd(const QModelIndex& index) const;
    Q_INVOKABLE quint8 cpuPercentage(const QModelIndex& index) const;
    Q_INVOKABLE float cpuUsage(const QModelIndex& index) const;
    Q_INVOKABLE quint64 memory(const QModelIndex& index) const;
    Q_INVOKABLE QString name(const QModelIndex& index) const;
    Q_INVOKABLE quint32 pid(const QModelIndex& index) const;
    Q_INVOKABLE quint32 uid(const QModelIndex& index) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
    void activeChanged();
};

class TimeSeries : public QAbstractItemModel
{
    Q_OBJECT
    friend class Demo;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    explicit TimeSeries(bool owned, QObject *parent);
public:
    explicit TimeSeries(QObject *parent = nullptr);
    ~TimeSeries();

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE float cos(int row) const;
    Q_INVOKABLE bool setCos(int row, float value);
    Q_INVOKABLE float sin(int row) const;
    Q_INVOKABLE bool setSin(int row, float value);
    Q_INVOKABLE float time(int row) const;
    Q_INVOKABLE bool setTime(int row, float value);

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
};
#endif // BINDINGS_H
