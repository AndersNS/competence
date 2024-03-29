var myChart = {};
export function renderChart(element, data) {
  const configSample = {
    type: 'radar',
    data: data,
    options: {
      responsive: true,
      plugins: {
        title: {
          display: false,
          text: '',
        },
        legend: {
          labels: {
            color: 'white',
          },
        },
      },
      scales: {
        r: {
          grid: {
            color: 'gray',
            offset: true,
          },
          angleLines: {
            color: 'gray',
          },
          title: {
            color: 'yellow',
          },
          ticks: {
            color: 'white',
            showLabelBackdrop: false,
            z: 1,
          },
          pointLabels: {
            color: 'white',
          },
        },
      },
      scale: {
        beginAtZero: true,
        min: 0,
        max: 5,
        ticks: {
          stepSize: 1,
        },
      },
    },
  };

  if (myChart[element.id]) {
    myChart[element.id].data = data;
    myChart[element.id].update();
    return;
  }
  myChart[element.id] = new Chart(element, configSample);
  return myChart;
}
